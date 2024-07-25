use std::collections::HashSet;

use convert_case::{Boundary, Case, Casing};
use reqwest::Method;

use crate::generator::param::Parameter;
use crate::generator::rust_type::RustType;
use crate::generator::utils::{extract_version_number, get_object_name_in_type};

pub const ENDPOINT_TEMPLATE: &str = r#"
pub struct {struct_name} {{description_and_fields}}

impl IsApiRequest for {struct_name} {
    const METHOD: Method = Method::{method};
    type ReturnType = {return_type};
    fn get_url(&self) -> String {{get_url}}{get_body}{get_query}
}

pub fn {fn_name}({fn_params}) -> {struct_name} {
    {struct_name}{{init_params}}
}
"#;


pub const ENDPOINT_GET_BODY_TEMPLATE: &str = r#"
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }"#;

pub const ENDPOINT_GET_QUERY_TEMPLATE: &str = r#"
    fn get_query(&self) -> Option<Vec<(String,String)>> {
        Some(vec![
{query_params}
        ])
    }"#;





#[derive(Debug, Clone)]
pub struct Endpoint {
    pub name: String,
    pub plugin: String,
    pub version: Option<String>,
    pub related_objs: HashSet<String>,
    pub path: String,
    pub method: reqwest::Method,
    pub description: Option<String>,
    pub route_parameters: Vec<Parameter>,
    pub queries: Vec<Parameter>,
    pub response: Option<RustType>,
    pub body: Option<RustType>,
}

impl Endpoint {
    pub fn new(operation_id: Option<String>, plugins: Vec<String>, path: String, method: reqwest::Method, description: Option<String>, parameters: Vec<Parameter>, queries: Vec<Parameter>, response: Option<RustType>, body: Option<RustType>) -> Endpoint {
        // check if path contain VX where X can be any number
        let mut related_objs = HashSet::new();
        for param in &parameters {
            if let Some(object_name) = get_object_name_in_type(&param.type_) {
                related_objs.insert(object_name);
            }
        }
        for query in &queries {
            if let Some(object_name) = get_object_name_in_type(&query.type_) {
                related_objs.insert(object_name);
            }
        }
        if let Some(body) = &body {
            if let Some(object_name) = get_object_name_in_type(&body) {
                related_objs.insert(object_name);
            }
        }

        if let Some(response) = &response {
            if let Some(object_name) = get_object_name_in_type(&response) {
                related_objs.insert(object_name);
            }
        }

        let mut plugin = plugins.first().unwrap().to_string().to_case(Case::Snake);
        Endpoint {
            name: operation_id.unwrap(),
            plugin,
            version: extract_version_number(&path),
            related_objs,
            path,
            method,
            description,
            route_parameters: parameters,
            queries,
            response,
            body,
        }
    }

    pub fn get_clean_path(&self) -> String {
        // remove what's inside the curly braces
        let mut new_path = self.path.clone();
        let mut start = None;
        let mut index = 0;
        while index < new_path.len() {
            let c = new_path.chars().nth(index).unwrap();
            if c == '{' {
                start = Some(index + 1);
            } else if c == '}' {
                if let Some(s) = start {
                    new_path.replace_range(s..index, "");
                    start = None;
                    index = s;
                }
            }
            index += 1;
        }
        new_path
    }
}


impl std::fmt::Display for Endpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fields = Vec::new();
        let mut fn_params = Vec::new();
        let mut init_params = Vec::new();

        let response_type = self.response.as_ref().unwrap_or(&RustType::Object(None));
        let method = reqwest_method_to_string(&self.method);

        let get_url = if !self.route_parameters.is_empty(){
            let mut url_params =Vec::new();
            for param in self.route_parameters.iter() {
                fields.push(param.to_string());
                fn_params.push(format!("{}: {}", param.get_name(), param.get_type_string()));
                init_params.push(format!("{}", param.get_name()));
                url_params.push(match param.type_ {
                    RustType::Object(_) => {
                        format!("serde_json::to_string(&self.{}).unwrap()", param.get_name())
                    }
                    _ => {
                        format!("self.{}", param.get_name())
                    }
                })
            }
            format!("format!(\"{}\", {})", self.get_clean_path(), url_params.join(", "))
        }else{
            format!("\"{}\".to_string()", self.get_clean_path())
        };


        let get_query = if !self.queries.is_empty(){
            let mut query_params = vec![];
            for query in self.queries.iter() {
                let real_name = query.get_name();
                fields.push(format!("    pub {}: {},", real_name, query.get_type_string()));
                fn_params.push(format!("{}: {}", real_name, query.get_type_string()));
                init_params.push(format!("{}", real_name));
                query_params.push( format!("            (\"{}\".to_string(), serde_json::to_string(&self.{}).unwrap())", query.name, real_name));
            }
            ENDPOINT_GET_QUERY_TEMPLATE.replace("{query_params}",&query_params.join(",\n"))

        }else{
            "".to_string()
        };


        let get_body = if let Some(body) = &self.body {
            fields.push(format!("    pub body: {},", body));
            fn_params.push(format!("body: {}", body));
            init_params.push("body".to_string());
            ENDPOINT_GET_BODY_TEMPLATE.to_string()
        }else{
            "".to_string()
        };

        let mut description_and_fields = if !fields.is_empty() {
            fields.join("\n")
        } else {
            "".to_string()
        };

        if let Some(desc) = &self.description {
            if !desc.is_empty() {
                description_and_fields = format!("{}\n{}", desc.split("\n").map(|x| format!("    /// {}", x)).collect::<String>(), description_and_fields);
            }
        }

        if !description_and_fields.is_empty() {
            description_and_fields = format!("\n{}\n", description_and_fields);
        }

        ENDPOINT_TEMPLATE.replace("{struct_name}", &self.name)
            .replace("{description_and_fields}", &description_and_fields)
            .replace("{get_url}", &get_url)
            .replace("{get_body}", &get_body)
            .replace("{get_query}", &get_query)
            .replace("{return_type}", &response_type.to_string())
            .replace("{method}", &method)
            .replace("{fn_name}", &self.name.clone().from_case(Case::Camel).without_boundaries(&[Boundary::UpperDigit]).to_case(Case::Snake))
            .replace("{fn_params}", &fn_params.join(", "))
            .replace("{init_params}", &init_params.join(", "))
            .fmt(f)
    }
}

pub fn reqwest_method_to_string(method: &Method) -> String{
    match *method {
        Method::GET => "GET".to_string(),
        Method::POST => "POST".to_string(),
        Method::PUT => "PUT".to_string(),
        Method::DELETE => "DELETE".to_string(),
        Method::HEAD => "HEAD".to_string(),
        Method::OPTIONS => "OPTIONS".to_string(),
        Method::CONNECT => "CONNECT".to_string(),
        Method::PATCH => "PATCH".to_string(),
        _ => panic!("Unsupported method: {:?}", method),
    }
}

