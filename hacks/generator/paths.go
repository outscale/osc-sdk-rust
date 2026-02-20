package main

import "fmt"

type PathTemplate struct {
	inner []PathSegment
}

type PathSegment struct {
	TemplateSegement *string
	Literal          *string
}

func NewTemplateSegement(t string) PathSegment {
	return PathSegment{
		TemplateSegement: &t,
		Literal:          nil,
	}
}

func NewLiteral(l string) PathSegment {
	return PathSegment{
		TemplateSegement: nil,
		Literal:          &l,
	}
}

func (pt *PathTemplate) AddSegment(seg PathSegment) {
	pt.inner = append(pt.inner, seg)
}

func Parse(input string) (*PathTemplate, error) {
	pt := &PathTemplate{}

	var currentSegement string
	var inTemplate bool
	var isTemplate bool

	for i, c := range input {
		switch c {
		case '/':
			if inTemplate {
				return nil, fmt.Errorf("unexpected / pos %d", i)
			}

			if len(currentSegement) > 0 {
				if isTemplate {
					pt.AddSegment(NewTemplateSegement(currentSegement))
				} else {
					pt.AddSegment(NewLiteral(currentSegement))
				}
				currentSegement = ""
				isTemplate = false
			}
		case '{':
			if len(currentSegement) > 0 {
				return nil, fmt.Errorf("template should start with { pos %d", i)
			}

			inTemplate = true
		case '}':
			if inTemplate {
				inTemplate = false
				isTemplate = true
			} else {
				return nil, fmt.Errorf("unexpected } pos %d", i)
			}
		default:
			if !isTemplate {
				currentSegement += string(c)
			} else {
				return nil, fmt.Errorf("unexpected char %c", c)
			}
		}
	}

	if inTemplate {
		return nil, fmt.Errorf("unmatched opening brace")
	}

	if len(currentSegement) > 0 {
		if isTemplate {
			pt.AddSegment(NewTemplateSegement(currentSegement))
		} else {
			pt.AddSegment(NewLiteral(currentSegement))
		}
	}

	return pt, nil
}

func (pt *PathTemplate) Generate(q []Parameter) string {
	impl := "let url = {\n let mut url = self.base_url.clone();\nurl.path_segments_mut().map_err(|_| ApiError::InvalidBaseUrl)?.extend(&["
	for i, s := range pt.inner {
		if i != 0 {
			impl += ", "
		}

		if s.Literal != nil {
			impl += "\"" + *s.Literal + "\""
		} else if s.TemplateSegement != nil {
			impl += *s.TemplateSegement + ".as_ref()"
		}
	}
	impl += "]);\n"

	if len(q) > 0 {
		impl += "{\nlet mut pairs = url.query_pairs_mut();\nlet serializer = serde_urlencoded::Serializer::new(&mut pairs);\n"
		impl += "if let Ok(mut ser_seq) = serializer.serialize_seq(None) {\n"

		for _, q := range q {
			if q.Required {
				impl += "ser_seq.serialize_element(&(\"" + q.Name + "\", params." + q.Name + "))?;\n"
			} else {
				impl += "if let Some(v) = params." + q.Name + " {\n"
				impl += "\tser_seq.serialize_element(&(\"" + q.Name + "\", v))?;\n"
				impl += "}\n"
			}
		}

		impl += "ser_seq.end()?;\n}\n}\n"
	}

	impl += "url\n};"

	return impl
}
