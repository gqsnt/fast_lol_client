use std::collections::HashSet;
use convert_case::{Case, Casing};
use reqwest::Method;
use crate::generator::param::Parameter;
use crate::generator::rust_type::RustType;
use crate::generator::utils::{crate_path_from_type_and_name, extract_version_number, get_object_name_in_type};


pub const ENDPOINT_TEMPLATE: &str = r#"
pub struct {struct_name} {
{description}
{fields}}

impl IsApiRequest for {struct_name} {
    const METHOD: Method = Method::{method};
    type ReturnType = {return_type};

    fn get_url(&self) -> String {
        {get_url}
    }

    fn get_body(&self) -> Option<Value> {
        {get_body}
    }

    fn get_query_params(&self) -> Option<Value> {
        {get_query_params}
    }
}

pub fn {fn_name}({fn_params}) -> {struct_name} {
    {struct_name} {
        {init_params}
    }
}
"#;


#[derive(Debug, Clone)]
pub struct Endpoint {
    pub name: String,
    pub plugin: String,
    pub version: Option<String>,
    pub related_objs: HashSet<String>,
    pub path: String,
    pub method: reqwest::Method,
    pub description: Option<String>,
    pub parameters: Vec<Parameter>,
    pub queries : Vec<Parameter>,
    pub response: Option<RustType>,
    pub body: Option<RustType>,
}

impl Endpoint{
    pub fn new(operation_id: Option<String>, plugins: Vec<String>, path: String, method: reqwest::Method, description: Option<String>, parameters: Vec<Parameter>, queries: Vec<Parameter>, response: Option<RustType>, body: Option<RustType>) -> Endpoint {
        // check if path contain VX where X can be any number
        let mut related_objs = HashSet::new();
        for param in &parameters{
            if let Some(object_name) = get_object_name_in_type(&param.type_){
                related_objs.insert(object_name);
            }
        }
        for query in &queries{
            if let Some(object_name) = get_object_name_in_type(&query.type_){
                related_objs.insert(object_name);
            }
        }
        if let Some(body) = &body{
            if let Some(object_name) = get_object_name_in_type(&body){
                related_objs.insert(object_name);
            }
        }

        if let Some(response) = &response{
            if let Some(object_name) = get_object_name_in_type(&response){
                related_objs.insert(object_name);
            }
        }

        let mut plugin = plugins.first().unwrap().to_string();
        Endpoint {
            name:operation_id.unwrap(),
            plugin: plugin.to_case(Case::Snake),
            version:extract_version_number(&path),
            related_objs: related_objs,
            path,
            method,
            description,
            parameters,
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
                start = Some(index+1);
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
        let mut fields = String::new();
        for param in self.parameters.iter() {
            fields.push_str(&format!("{}", param.to_string()));
        }
        for query in self.queries.iter() {
            fields.push_str(&format!("{}", query.to_string()));
        }
        if let Some(body) = &self.body{
            fields.push_str(format!("    pub body: {},\n", body).as_str())
        }
        let description = if let Some(description) = &self.description {
            if !description.is_empty() {
                description.split("\n").map(|x| format!("    // {}", x)).collect::<String>()
            } else {
                "".to_string()
            }
        } else {
            "".to_string()
        };

        let struct_name = self.name.clone();


        let url_params = self.parameters.iter().map(|x| {
            match x.type_{
                RustType::Object(_) => {
                    format!("serde_json::to_string(&self.{}).unwrap()",x.get_name())
                }
                _ => {
                    format!("self.{}",x.get_name())
                }
            }
        }).collect::<Vec<String>>().join(", ");

        let get_url = if ! url_params.is_empty(){
            format!("format!(\"{}\", {})", self.get_clean_path(), url_params)
        }else {
            format!("\"{}\".to_string()", self.get_clean_path())
        };


        let get_body = if let Some(body) = &self.body{
            "Some(to_value(&self.body).unwrap())"
        }else{
            "None"
        };
        let get_query_params = if !self.queries.is_empty(){
            let mut query_params = String::from("Some(json!({\n");
            for query in self.queries.iter(){
                query_params.push_str(format!("            \"{}\" : self.{},\n", query.name, query.get_name()).as_str());
            }
            query_params.push_str("        }))");
            query_params
        }else{
            "None".to_string()
        };


        let response_type = self.response.as_ref().unwrap_or(&RustType::Object(None));
        let method = reqwest_method_to_string(&self.method);


        let mut helper_fn_name = self.name.clone().to_case(Case::Snake);

        let mut helper_fn_params = vec![];

        for param in &self.parameters {
            helper_fn_params.push(format!("{}: {}", param.get_name(), param.type_));
        }

        for query in &self.queries {
            helper_fn_params.push(format!("{}: {}", query.get_name(), query.type_));
        }

        if let Some(body) = &self.body {
            helper_fn_params.push(format!("body: {}", body));
        }


        let mut helper_fn_params_struct = vec![];

        for param in &self.parameters {
            helper_fn_params_struct.push(format!("{}", param.get_name()));
        }
        for query in &self.queries {
            helper_fn_params_struct.push(format!("{}", query.get_name()));
        }
        if let Some(_) = &self.body {
            helper_fn_params_struct.push("body".to_string());
        }

        ENDPOINT_TEMPLATE.replace("{struct_name}", &struct_name)
            .replace("{description}", &description)
            .replace("{fields}", &fields)
            .replace("{get_url}", &get_url)
            .replace("{get_body}", get_body)
            .replace("{get_query_params}", get_query_params.as_str())
            .replace("{return_type}", &response_type.to_string())
            .replace("{method}", &method)
            .replace("{fn_name}", &helper_fn_name)
            .replace("{fn_params}", &helper_fn_params.join(", "))
            .replace("{init_params}", &helper_fn_params_struct.join(", "))
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

