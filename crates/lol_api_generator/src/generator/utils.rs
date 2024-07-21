use openapiv3::{ArrayType, ParameterData, ParameterSchemaOrContent, ReferenceOr, SchemaKind, StatusCode};
use convert_case::{Case, Casing};
use syn::parse::{Parser, ParseStream};
use syn::Ident;
use regex::Regex;
use crate::generator::param::Parameter;
use crate::generator::rust_type::RustType;


pub fn get_object_name_in_type(type_:&RustType) -> Option<String>{
    match type_{
        RustType::Object(Some(object)) => {
            Some(object.clone())
        }
        RustType::Array(inner) => {
            if let RustType::Object(Some(name)) = *inner.clone(){
                return Some(name.clone());
            }
            None
        }
        _ => {
            None
        }
    }
}


pub fn parse_reference(reference: &str) -> Option<String> {
    let mut parts = reference.split("/");
    let new_reference = parts.last().unwrap().to_string();
    if !new_reference.is_empty() {
        return Some(format_struct_name(&new_reference));
    }
    None
}

pub fn get_child_schema(array_type: ArrayType) -> RustType {
    match array_type.items.unwrap() {
        ReferenceOr::Reference { reference } => {
            return RustType::Object(parse_reference(reference.as_str()));
        }
        ReferenceOr::Item(i) => {
            match i.schema_kind {
                SchemaKind::Type(type_) => {
                    return type_.into();
                }
                _ => {
                    panic!("Array type not supported");
                }
            }
        }
    }
}

pub fn parse_parameters_queries(parameters: &Vec<ReferenceOr<openapiv3::Parameter>>) -> (Vec<Parameter>, Vec<Parameter>) {
    let mut new_parameters = vec![];
    let mut new_queries = vec![];
    for parameter in parameters {
        if let ReferenceOr::Item(parameter) = parameter {
            match parameter {
                openapiv3::Parameter::Query { parameter_data, allow_reserved, style, allow_empty_value } => {
                    if let Some(parameter) = parameter_data_to_parameter(parameter_data) {
                        new_queries.push(parameter);
                    }
                }
                openapiv3::Parameter::Path { parameter_data, style } => {
                    if let Some(parameter) = parameter_data_to_parameter(parameter_data) {
                        new_parameters.push(parameter);
                    }
                }
                _ => {
                    panic!("Parameter not supported");
                }
            }
        } else {
            panic!("ReferenceOr::Reference not supported");
        }
    }
    (new_parameters, new_queries)
}

pub fn parameter_data_to_parameter(parameter_data: &ParameterData) -> Option<Parameter> {
    match &parameter_data.format {
        ParameterSchemaOrContent::Schema(schema) => {
            return match schema {
                ReferenceOr::Reference { reference } => {
                    Some(Parameter::new(parameter_data.name.clone(), parameter_data.required, RustType::Object(parse_reference(reference.as_str()))))
                }
                ReferenceOr::Item(schema) => {
                    match &schema.schema_kind {
                        SchemaKind::Type(type_) => {
                            Some(Parameter::new(parameter_data.name.clone(), parameter_data.required, type_.clone().into()))
                        }
                        _ => { panic!("SchemaKind not supported"); }
                    }
                }
            };
        }
        ParameterSchemaOrContent::Content(_) => {
            panic!("ParameterSchemaOrContent::Content not supported");
        }
    }
    None
}

pub fn parse_response(responses: &openapiv3::Responses) -> Option<RustType> {
    for (status_code, response) in responses.responses.iter() {
        if *status_code == StatusCode::Range(2) {
            if let ReferenceOr::Item(response) = response {
                for (content_type, media_type) in response.content.iter() {
                    if content_type == "application/json" {
                        if let Some(schema) = &media_type.schema {
                            match schema {
                                ReferenceOr::Reference { reference } => {
                                    return Some(RustType::Object(parse_reference(reference.as_str())));
                                }
                                ReferenceOr::Item(schema) => {
                                    if let SchemaKind::Type(type_) = &schema.schema_kind {
                                        return Some(type_.clone().into());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

pub fn parse_body(request_body: &Option<ReferenceOr<openapiv3::RequestBody>>) -> Option<RustType> {
    if let Some(request_body) = request_body {
        if let ReferenceOr::Item(request_body) = request_body {
            for (content_type, media_type) in request_body.content.iter() {
                if content_type == "application/json" {
                    if let Some(schema) = &media_type.schema {
                        match schema {
                            ReferenceOr::Reference { reference } => {
                                return Some(RustType::Object(parse_reference(reference.as_str())));
                            }
                            ReferenceOr::Item(schema) => {
                                if let SchemaKind::Type(type_) = &schema.schema_kind {
                                    return Some(type_.clone().into());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

pub fn format_struct_name(name: &str) -> String {
    name.replace("-", "").to_case(Case::Pascal)
}

pub fn is_rust_keyword(keyword: &str) -> bool {
    let parser = |input: ParseStream| -> syn::Result<Ident> {
        input.parse::<Ident>()
    };

    parser.parse_str(keyword).is_err()
}


pub fn extract_version_number(s: &str) -> Option<String> {
    let re = Regex::new(r"/v\d+/").unwrap();
    if let Some(matched) = re.find(s) {
        Some(matched.as_str().to_string().replace("/", ""))
    } else {
        None
    }
}


pub fn crate_path_from_type_and_name(type_: &str, name: &str) -> String {
    let lower_name = name.to_case(Case::Snake);
    format!("crate::{}::{}::{}", type_,lower_name, name)
}


