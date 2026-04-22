package main

import (
	"fmt"
	"slices"
	"strconv"
	"strings"

	"github.com/getkin/kin-openapi/openapi3"
	"github.com/iancoleman/strcase"
)

type Type struct {
	Name           string
	Implementation string
	Nullable       bool
	Description    string
	WasDumped      bool
}

// formatDocComment formats a description string as a Rust doc comment
// It handles multi-line descriptions and ensures proper Rust doc comment formatting
func formatDocComment(description string) string {
	if description == "" {
		return ""
	}

	lines := strings.Split(description, "\n")
	var docLines []string
	for _, line := range lines {
		docLines = append(docLines, "/// "+line)
	}
	return strings.Join(docLines, "\n") + "\n"
}

func (t *Type) NullableName(nullable bool) string {
	if t.Nullable || nullable {
		return "std::option::Option<" + t.Name + ">"
	}
	return t.Name
}

func (t *Type) FullName() string {
	return t.NullableName(false)
}

type State struct {
	inner map[string]*Type
}

func NewState() State {
	return State{
		inner: make(map[string]*Type),
	}
}

// AddRef is the single public entry point for type resolution and caching.
// When scr carries a $ref, that ref path is used as the canonical cache key,
// guaranteeing that any schema referenced multiple times across the spec
// resolves to the same *Type. For inline schemas the key is derived from the
// supplied name path.
func (s *State) AddRef(
	scr *openapi3.SchemaRef,
	path *string,
) (*Type, error) {
	var key string
	if scr.Ref != "" {
		key = scr.Ref
	} else if path != nil && *path != "" {
		key = *path
	}

	if t, ok := s.inner[key]; ok {
		return t, nil
	}

	t, err := s.addResolved(scr.Value, key)
	if err != nil {
		return nil, err
	}

	if key != "" {
		s.inner[key] = t
	}
	return t, nil
}

func (s *State) Dump() string {
	var out string
	for k, t := range s.inner {
		if t.WasDumped {
			continue
		}
		out += "// key: " + k + "\n"
		out += t.Implementation + "\n\n"
		t.WasDumped = true
	}
	return out
}

func (s *State) addResolved(sc *openapi3.Schema, path string) (*Type, error) {
	if value, ok := sc.Extensions["x-rs-type"]; ok {
		casted, ok := value.(string)
		if ok {
			return &Type{
				Name:        casted,
				Description: formatDocComment(sc.Description),
			}, nil
		}
	}

	if len(sc.OneOf) > 0 {
		return s.addOneOf(sc, path)
	}

	if len(sc.AllOf) > 0 {
		return s.addAllOf(sc, path)
	}

	if len(sc.AnyOf) > 0 {
		return s.addAnyOf(sc, path)
	}

	var returnType *Type
	var err error
	nullable := sc.Nullable
	for _, t := range *sc.Type {
		switch t {
		case "boolean":
			returnType, err = s.addBoolean(sc)
		case "object":
			returnType, err = s.addObject(sc, path)
		case "array":
			returnType, err = s.addArray(sc, path)
		case "string":
			returnType, err = s.addString(sc, path)
		case "number":
			returnType, err = s.addNumber(sc)
		case "integer":
			returnType, err = s.addInteger(sc)
		case "null":
			nullable = true
		default:
			return nil, fmt.Errorf("unsupported type '%s' for schema %s", t, path)
		}

		if err != nil {
			return nil, err
		}
	}

	if nullable {
		copyType := Type{
			Name:           returnType.Name,
			Implementation: returnType.Implementation,
			Description:    returnType.Description,
			Nullable:       true,
		}
		returnType = &copyType
	}

	return returnType, nil
}

func (s *State) addBoolean(sc *openapi3.Schema) (*Type, error) {
	return &Type{
		Name:           "bool",
		Implementation: "",
	}, nil
}

func keywordsFix(s string) string {
	switch {
	case s == "type":
		return "r#type"
	default:
		return s
	}
}

func (s *State) addObject(sc *openapi3.Schema, path string) (*Type, error) {
	var props []string

	if sc.Properties == nil {
		childPath := path + "/Value"
		t, err := s.AddRef(sc.AdditionalProperties.Schema, &childPath)
		if err != nil {
			return nil, err
		}

		return &Type{
			Name: "::std::collections::HashMap<::std::string::String, " + t.NullableName(
				false,
			) + ">",
			Implementation: "",
		}, nil
	}

	for name, scc := range sc.Properties {
		childPath := path + "/" + name
		propName, err := s.AddRef(scc, &childPath)
		if err != nil {
			return nil, fmt.Errorf("could not resolve prop type: %v", err)
		}

		// Add description as doc comment for the field if available
		var fieldDocs string
		if scc.Value != nil && scc.Value.Description != "" {
			fieldDocs = formatDocComment(scc.Value.Description)
		}

		prop := fmt.Sprintf(
			"%s#[serde(rename = \"%s\")]\n\tpub %s: %s,",
			fieldDocs,
			name,
			keywordsFix(strcase.ToSnake(name)),
			propName.NullableName(!slices.Contains(sc.Required, name)),
		)
		props = append(props, prop)
	}

	name := nameFromPath(path, sc)

	// Add struct-level documentation
	structDocs := "/// path: " + path + "\n"
	if sc.Description != "" {
		structDocs = formatDocComment(sc.Description)
	}

	return &Type{
		Name:        name,
		Description: sc.Description,
		Implementation: structDocs + fmt.Sprintf(
			"#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]\n#[serde(default)]\npub struct %s {\n%s\n}",
			name,
			strings.Join(props, "\n"),
		),
	}, nil
}

func (s *State) addArray(sc *openapi3.Schema, path string) (*Type, error) {
	if sc.Items == nil {
		return nil, fmt.Errorf("array without items is not supported")
	}

	itemsStr := path + "/Items"
	itemsName, err := s.AddRef(sc.Items, &itemsStr)
	if err != nil {
		return nil, fmt.Errorf("could not resolve item type: %v", err)
	}

	name := "::std::vec::Vec<" + itemsName.FullName() + ">"

	return &Type{
		Name:           name,
		Implementation: "",
	}, nil
}

func (s *State) addString(sc *openapi3.Schema, path string) (*Type, error) {
	// Check if this is an enum
	if len(sc.Enum) > 0 {
		return s.addStringEnum(sc, path)
	}

	switch sc.Format {
	case "date-time", "datetime":
		return &Type{
			Name:           "::chrono::DateTime<chrono::Utc>",
			Implementation: "",
		}, nil
	case "binary":
		return &Type{
			Name:           "::std::vec::Vec<u8>",
			Implementation: "",
		}, nil
	default:
		return &Type{
			Name:           "::std::string::String",
			Implementation: "",
		}, nil
	}
}

func (s *State) addStringEnum(sc *openapi3.Schema, path string) (*Type, error) {
	name := nameFromPath(path, sc)

	var variants []string

	var defaultValue string
	if defValue, ok := sc.Default.(string); ok {
		defaultValue = strcase.ToCamel(defValue)
	}

	for _, enumVal := range sc.Enum {
		strVal, ok := enumVal.(string)
		if !ok {
			return nil, fmt.Errorf("non-string enum value in string enum for schema %s", path)
		}

		// Convert to valid Rust identifier
		variantName := strcase.ToCamel(strVal)

		if defaultValue == "" {
			defaultValue = strcase.ToCamel(strVal)
		}

		// If the variant name differs from the original value, add serde rename
		var variant string
		if variantName != strVal {
			variant = fmt.Sprintf("\t#[serde(rename = \"%s\")]\n\t%s,", strVal, variantName)
		} else {
			variant = fmt.Sprintf("\t%s,", variantName)
		}

		variants = append(variants, variant)
	}

	// Add enum-level documentation
	var enumDocs string
	if sc.Description != "" {
		enumDocs = formatDocComment(sc.Description)
	}

	impl := fmt.Sprintf(
		`%s#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum %s {
%s
}

#[allow(clippy::derivable_impls)]
impl ::std::default::Default for %s {
	fn default() -> Self {
		Self::%s
	}
}`,
		enumDocs,
		name,
		strings.Join(variants, "\n"),
		name,
		defaultValue,
	)

	return &Type{
		Name:           name,
		Description:    sc.Description,
		Implementation: impl,
	}, nil
}

func (s *State) addNumber(sc *openapi3.Schema) (*Type, error) {
	// OpenAPI "number" type is for floating-point numbers
	switch sc.Format {
	case "float":
		return &Type{
			Name:           "f32",
			Implementation: "",
		}, nil
	case "double":
		return &Type{
			Name:           "f64",
			Implementation: "",
		}, nil
	default:
		// Default to f64 for maximum precision when format is unspecified
		return &Type{
			Name:           "f64",
			Implementation: "",
		}, nil
	}
}

func (s *State) addInteger(sc *openapi3.Schema) (*Type, error) {
	// OpenAPI "integer" type - format can specify the size
	switch sc.Format {
	case "int32":
		return &Type{
			Name:           "i32",
			Implementation: "",
		}, nil
	case "int64":
		return &Type{
			Name:           "i64",
			Implementation: "",
		}, nil
	default:
		// Default to i32 for compatibility when format is unspecified
		return &Type{
			Name:           "i32",
			Implementation: "",
		}, nil
	}
}

func isNull(sc *openapi3.Schema) bool {
	t := sc.Type.Slice()
	if len(t) == 1 && t[0] == "null" {
		return true
	}
	return false
}

func (s *State) addOneOf(sc *openapi3.Schema, path string) (*Type, error) {
	var hasNull bool
	var types []*Type
	for i, scr := range sc.OneOf {
		if isNull(scr.Value) {
			hasNull = true
			continue
		}

		next := path + "/" + strconv.Itoa(i)
		t, err := s.AddRef(scr, &next)
		if err != nil {
			return nil, err
		}

		types = append(types, t)
	}

	if len(types) == 1 {
		types[0].Nullable = (types[0].Nullable) || hasNull
		return types[0], nil
	} else if len(types) > 1 && hasNull {
		return nil, fmt.Errorf("oneOf with multiple types and null is not supported for schema %s", path)
	}

	name := nameFromPath(path, sc)
	var variants []string
	for _, t := range types {
		// Use the type name as the variant name, not duplicate it
		variants = append(variants, fmt.Sprintf("\t%s(%s),", t.Name, t.FullName()))
	}

	impl := fmt.Sprintf(
		`#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum %s {
%s
	Unknown(::serde_json::Value),
}

#[allow(clippy::derivable_impls)]
impl ::std::default::Default for %s {
	fn default() -> Self {
		Self::Unknown(::serde_json::Value::Null)
	}
}`,
		name,
		strings.Join(variants, "\n"),
		name,
	)

	return &Type{
		Name:           name,
		Implementation: impl,
	}, nil
}

func (s *State) addAnyOf(sc *openapi3.Schema, path string) (*Type, error) {
	var hasNull bool
	var types []*Type
	for i, scr := range sc.AnyOf {
		if isNull(scr.Value) {
			hasNull = true
			continue
		}

		next := path + "/" + strconv.Itoa(i)
		t, err := s.AddRef(scr, &next)
		if err != nil {
			return nil, err
		}

		types = append(types, t)
	}

	if len(types) == 1 {
		types[0].Nullable = (types[0].Nullable) || hasNull
		return types[0], nil
	} else if len(types) > 1 && hasNull {
		return nil, fmt.Errorf("anyOf with multiple types and null is not supported for schema %s", path)
	}

	// anyOf with multiple types - treat similar to oneOf for now
	// This is technically incorrect (anyOf allows multiple to match)
	// but Rust's type system doesn't have a clean way to represent this
	name := nameFromPath(path, sc)

	var variants []string
	for i, t := range types {
		variants = append(variants, fmt.Sprintf("\t%s%d(%s),", name, i, t.FullName()))
	}

	impl := fmt.Sprintf(
		`#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum %s {
%s
	Unknown(::serde_json::Value),
}

#[allow(clippy::derivable_impls)]
impl ::std::default::Default for %s {
	fn default() -> Self {
		Self::Unknown(::serde_json::Value::Null)
	}
}`,
		name,
		strings.Join(variants, "\n"),
		name,
	)

	return &Type{
		Name:           name,
		Implementation: impl,
	}, nil
}

func (s *State) addAllOf(sc *openapi3.Schema, path string) (*Type, error) {
	var hasNull bool
	var schemas []openapi3.SchemaRef

	// Collect all non-null schemas
	for _, scr := range sc.AllOf {
		if isNull(scr.Value) {
			hasNull = true
			continue
		}

		schemas = append(schemas, *scr)
	}

	if hasNull {
		return nil, fmt.Errorf("allOf with null type is not supported for schema %s", path)
	}

	if len(schemas) == 0 {
		return nil, fmt.Errorf("allOf must have at least one schema for %s", path)
	}

	// If only one schema, just use it directly
	if len(schemas) == 1 {
		return s.AddRef(&schemas[0], &path)
	}

	return nil, fmt.Errorf("allOf is not supported. BY DESIGN")
}

func nameFromPath(path string, sc *openapi3.Schema) string {
	if value, ok := sc.Extensions["x-rs-name"]; ok {
		casted, ok := value.(string)
		if ok {
			return casted
		}
	}

	// ex: #/components/schemas/user -> User
	// ex: #/components/schemas/user/roles/Items -> UserRoles

	parts := strings.Split(path, "/")
	var name string
	for _, part := range parts {
		if part == "components" || part == "schemas" || part == "properties" || part == "Items" || part == "Value" || part == "" || part == "#" {
			continue
		}

		name += strcase.ToCamel(part)
	}

	return name
}
