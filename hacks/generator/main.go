package main

import (
	"bytes"
	"flag"
	"fmt"
	"io"
	"net/url"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"text/template"

	"github.com/getkin/kin-openapi/openapi3"
	"github.com/iancoleman/strcase"
	"github.com/speakeasy-api/openapi/overlay/loader"
	"gopkg.in/yaml.v3"
)

type Operation struct {
	Name            string
	Path            *PathTemplate
	Method          string
	PathParameters  []Parameter
	QueryParameters []Parameter
	RequestBody     *RequestBody
	ReturnValue     []ReturnValue
}

type Parameter struct {
	Name     string
	Required bool
	Schema   *openapi3.SchemaRef
	Path     string
}

type RequestBody struct {
	ContentType string
	Schema      *openapi3.SchemaRef
}

type ReturnValue struct {
	StatusCode  int
	ContentType string
	Schema      *openapi3.SchemaRef
}

func AggregateOperations(spec *openapi3.T) ([]Operation, error) {
	var operations []Operation

	for path, pathItem := range (*spec.Paths).Map() {
		pt, err := Parse(path)
		if err != nil {
			return nil, err
		}

		for method, operation := range pathItem.Operations() {
			op := Operation{
				Name:   normalizeOperationName(operation.OperationID, method, path),
				Path:   pt,
				Method: strings.ToUpper(method),
				PathParameters: aggregateParameters(
					pathItem.Parameters,
					operation.Parameters,
					"path",
				),
				QueryParameters: aggregateParameters(
					pathItem.Parameters,
					operation.Parameters,
					"query",
				),
				RequestBody: aggregateRequestBody(operation.RequestBody),
				ReturnValue: aggregateReturnValue(operation.Responses),
			}
			operations = append(operations, op)
		}
	}

	return operations, nil
}

func normalizeOperationName(operationId, method, path string) string {
	if operationId != "" {
		return strcase.ToSnake(operationId)
	}

	path = strings.Trim(path, "/")
	path = strings.ReplaceAll(path, "/", "_")
	return strcase.ToSnake(fmt.Sprintf("%s_%s", strings.ToLower(method), path))
}

func aggregateParameters(pathParams, params openapi3.Parameters, in string) []Parameter {
	paramsMap := make(map[string]*openapi3.ParameterRef)
	for _, p := range pathParams {
		if p.Value != nil && p.Value.In == in {
			paramsMap[p.Value.Name] = p
		}
	}
	for _, p := range params {
		if p.Value != nil && p.Value.In == in {
			paramsMap[p.Value.Name] = p
		}
	}

	var parameters []Parameter

	for _, param := range paramsMap {
		p := Parameter{
			Name:     param.Value.Name,
			Required: param.Value.Required,
			Schema:   param.Value.Schema,
			Path:     param.Ref,
		}
		parameters = append(parameters, p)
	}

	return parameters
}

func aggregateRequestBody(reqBody *openapi3.RequestBodyRef) *RequestBody {
	if reqBody == nil || reqBody.Value == nil {
		return nil
	}

	var contentType string
	var schema *openapi3.SchemaRef
	for ct, mediaType := range reqBody.Value.Content {
		contentType = ct
		if mediaType.Schema != nil {
			schema = mediaType.Schema
		}
		break // Assuming only one content type for simplicity
	}

	return &RequestBody{
		ContentType: contentType,
		Schema:      schema,
	}
}

func aggregateReturnValue(responses *openapi3.Responses) []ReturnValue {
	if responses == nil {
		return []ReturnValue{}
	}

	var returnValues []ReturnValue
	for code, resp := range (*responses).Map() {
		statusCode, _ := strconv.Atoi(code)

		for ct, mediaType := range resp.Value.Content {
			var schema *openapi3.SchemaRef
			if mediaType.Schema != nil {
				schema = mediaType.Schema
			}

			returnValue := ReturnValue{
				StatusCode:  statusCode,
				ContentType: ct,
				Schema:      schema,
			}
			returnValues = append(returnValues, returnValue)
		}

	}

	return returnValues
}

type Configuration struct {
	SpecPath string  `yaml:"spec"`
	Overlay  *string `yaml:"overlay"`
}

func ReadConfiguration(config *string) (*Configuration, error) {
	if config == nil || *config == "" {
		return nil, fmt.Errorf("configuration file path not provided")
	}

	source, err := os.ReadFile(*config)
	if err != nil {
		return nil, fmt.Errorf("error reading configuration file %s: %w", *config, err)
	}

	var cfg Configuration
	err = yaml.Unmarshal(source, &cfg)
	if err != nil {
		return nil, fmt.Errorf("error unmarshalling configuration: %w", err)
	}

	return &cfg, nil
}

func LoadSwagger(filePath string) (swagger *openapi3.T, err error) {
	loader := openapi3.NewLoader()
	loader.IsExternalRefsAllowed = true

	u, err := url.Parse(filePath)
	if err == nil && u.Scheme != "" && u.Host != "" {
		return loader.LoadFromURI(u)
	} else {
		return loader.LoadFromFile(filePath)
	}
}

func LoadSwaggerWithOverlay(cfg *Configuration) (swagger *openapi3.T, err error) {
	spec, err := LoadSwagger(cfg.SpecPath)
	if err != nil {
		return nil, fmt.Errorf("failed to load OpenAPI specification: %w", err)
	}

	if cfg.Overlay == nil {
		return spec, nil
	}

	// parse out the yaml.Node, which is required by the overlay library
	buf := &bytes.Buffer{}
	enc := yaml.NewEncoder(buf)
	// set to 2 to work around https://github.com/yaml/go-yaml/issues/76
	enc.SetIndent(2)
	err = enc.Encode(spec)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal spec from %#v as YAML: %w", cfg.SpecPath, err)
	}

	var node yaml.Node
	err = yaml.NewDecoder(buf).Decode(&node)
	if err != nil {
		return nil, fmt.Errorf("failed to parse spec from %#v: %w", cfg.SpecPath, err)
	}

	overlay, err := loader.LoadOverlay(*cfg.Overlay)
	if err != nil {
		return nil, fmt.Errorf("failed to load Overlay from %#v: %v", cfg.Overlay, err)
	}

	err = overlay.Validate()
	if err != nil {
		return nil, fmt.Errorf("the Overlay in %#v was not valid: %v", cfg.Overlay, err)
	}

	vs, err := overlay.ApplyToStrict(&node)
	if err != nil {
		return nil, fmt.Errorf("failed to apply Overlay %#v to specification %#v: %v\nAdditionally, the following validation errors were found:\n- %s", cfg.Overlay, cfg.SpecPath, err, strings.Join(vs, "\n- "))
	}

	b, err := yaml.Marshal(&node)
	if err != nil {
		return nil, fmt.Errorf("failed to serialize Overlay'd specification %#v: %v", cfg.Overlay, err)
	}

	loader := openapi3.NewLoader()
	loader.IsExternalRefsAllowed = true

	swagger, err = loader.LoadFromDataWithPath(b, &url.URL{
		Path: filepath.ToSlash(cfg.SpecPath),
	})
	if err != nil {
		return nil, fmt.Errorf("failed to serialize Overlay'd specification %#v: %v", cfg.Overlay, err)
	}

	return swagger, nil
}

// loadTemplate loads a template file from the templates directory
func loadTemplate(name string) (string, error) {
	// Try multiple locations for templates
	possiblePaths := []string{
		filepath.Join("templates", name+".tmpl"),                       // relative to current directory
		filepath.Join("hacks", "generator", "templates", name+".tmpl"), // from project root
		filepath.Join("..", "..", "templates", name+".tmpl"),           // relative from build output
	}

	var lastErr error
	for _, templatePath := range possiblePaths {
		content, err := os.ReadFile(templatePath)
		if err == nil {
			return string(content), nil
		}
		lastErr = err
	}

	return "", fmt.Errorf("failed to read template file %s.tmpl from any location: %w", name, lastErr)
}

type paramStructField struct {
	name string
	kind string
}

func genOpParamStruct(state *State, op *Operation) (string, error) {
	var fields []paramStructField
	for _, p := range op.PathParameters {
		if p.Required {
			continue
		}

		t, err := state.AddRef(p.Schema, &p.Path)
		if err != nil {
			return "", fmt.Errorf("failed to resolve path parameter %s: %w", p)
		}

		fields = append(fields, paramStructField{
			name: strcase.ToSnake(p.Name),
			kind: t.NullableName(false),
		})
	}
	for _, p := range op.QueryParameters {
		t, err := state.AddRef(p.Schema, &p.Path)
		if err != nil {
			return "", fmt.Errorf("failed to resolve query parameter %s: %w", p)
		}

		fields = append(fields, paramStructField{
			name: strcase.ToSnake(p.Name),
			kind: t.NullableName(!p.Required),
		})
	}

	if len(fields) == 0 {
		return "", nil
	}

	output := "#[derive(Debug, Clone, Default)]\npub struct " + strcase.ToCamel(op.Name) + "Param {\n"
	for _, f := range fields {
		output += "\tpub " + f.name + ": " + f.kind + ",\n"
	}
	output += "}"
	return output, nil
}

// createTemplateFuncs creates the template function map
func createTemplateFuncs(state *State) template.FuncMap {
	return template.FuncMap{
		"opParamStruct": func(op *Operation) (string, error) {
			return genOpParamStruct(state, op)
		},
		"opArgs": func(op *Operation) (string, error) {
			args := []string{"&mut self"}
			for _, p := range op.PathParameters {
				if !p.Required {
					continue
				}
				t, err := state.AddRef(p.Schema, &p.Path)
				if err != nil {
					return "", fmt.Errorf("failed to resolve path parameter %s: %w", p.Name, err)
				}
				args = append(args, fmt.Sprintf("%s: %s", p.Name, t.NullableName(false)))
			}
			paramImplem, err := genOpParamStruct(state, op)
			if err != nil {
				return "", fmt.Errorf("failed to generates struct: %w", err)
			}
			if paramImplem != "" {
				args = append(args, "params: "+strcase.ToCamel(op.Name)+"Param")
			}
			if op.RequestBody != nil {
				t, err := state.AddRef(op.RequestBody.Schema, nil)
				if err != nil {
					return "", fmt.Errorf("failed to resolve request body: %w", err)
				}
				args = append(args, "request_body: "+t.FullName())
			}
			return strings.Join(args, ", "), nil
		},
		"opRets": func(op *Operation) (string, error) {
			for _, r := range op.ReturnValue {
				if r.StatusCode >= 200 && r.StatusCode < 300 {
					t, err := state.AddRef(r.Schema, nil)
					if err != nil {
						return "", fmt.Errorf("failed to resolve response type: %w", err)
					}
					return fmt.Sprintf("::std::result::Result<%s, ApiError>", t.NullableName(false)), nil
				}
			}
			return "", fmt.Errorf("no 2xx response found for operation %s", op.Name)
		},
		"urlGen": func(op *Operation) string {
			return op.Path.Generate(op.QueryParameters)
		},
		"stateDump": func() string {
			return state.Dump()
		},
		"sucessRes": func(r *ReturnValue) bool {
			return r.StatusCode >= 200 && r.StatusCode < 300
		},
		"typeRes": func(r *ReturnValue, opName string) (string, error) {
			t, err := state.AddRef(r.Schema, nil)
			if err != nil {
				return "", fmt.Errorf("failed to resolve response type: %w", err)
			}
			return t.FullName(), nil
		},
	}
}

// generateCode generates Rust code from operations
func generateCode(operations []Operation, output io.Writer) error {
	state := NewState()

	templateContent, err := loadTemplate("operations")
	if err != nil {
		return fmt.Errorf("error loading template: %w", err)
	}

	tmpl, err := template.New("operations").Funcs(createTemplateFuncs(&state)).Parse(templateContent)
	if err != nil {
		return fmt.Errorf("error parsing template: %w", err)
	}

	err = tmpl.Execute(output, operations)
	if err != nil {
		return fmt.Errorf("error executing template: %w", err)
	}

	return nil
}

// run is the main application logic
func run(configPath, outputPath string) error {
	// Read configuration
	cfg, err := ReadConfiguration(&configPath)
	if err != nil {
		return fmt.Errorf("error reading configuration: %w", err)
	}

	// Load OpenAPI specification
	spec, err := LoadSwaggerWithOverlay(cfg)
	if err != nil {
		return fmt.Errorf("error loading OpenAPI spec: %w", err)
	}

	// Aggregate operations
	operations, err := AggregateOperations(spec)
	if err != nil {
		return fmt.Errorf("error aggregating operations: %w", err)
	}

	// Determine output destination
	var output io.Writer
	if outputPath == "" {
		output = os.Stdout
	} else {
		file, err := os.Create(outputPath)
		if err != nil {
			return fmt.Errorf("error creating output file %s: %w", outputPath, err)
		}
		defer file.Close()
		output = file
	}

	// Generate code
	if err := generateCode(operations, output); err != nil {
		return fmt.Errorf("error generating code: %w", err)
	}

	return nil
}

func main() {
	configPath := flag.String("config", "", "Configuration file path")
	outputPath := flag.String("output", "", "Output file path (default: stdout)")
	flag.Parse()

	if *configPath == "" {
		fmt.Fprintf(os.Stderr, "Error: --config flag is required\n")
		flag.Usage()
		os.Exit(1)
	}

	if err := run(*configPath, *outputPath); err != nil {
		fmt.Fprintf(os.Stderr, "Error: %v\n", err)
		os.Exit(1)
	}
}
