use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::path::PathBuf;
// use convert_case::{Case, Casing};
use openapiv3::{ReferenceOr, SchemaKind, Type};
use serde_json::Value;

use endpoint::Endpoint;
use enum_repr::Enum;
use object_repr::Object;
use param::Parameter;
use rust_type::RustType;

use crate::generator::plugin::Plugin;

pub mod endpoint;
pub mod enum_repr;
pub mod object_repr;
mod param;
pub mod rust_type;
pub mod utils;
mod plugin;

#[derive(Debug, Clone)]
pub enum ObjectType {
    Object(Object),
    Enum(Enum),
}

impl ObjectType {
    pub fn get_inner_name(&self) -> &str {
        match self {
            ObjectType::Object(object) => &object.name,
            ObjectType::Enum(enum_) => &enum_.name,
        }
    }

    pub fn get_imports(&self) -> Option<HashSet<String>> {
        match self {
            ObjectType::Object(object) => {
                if object.related_objs.is_empty() {
                    None
                } else {
                    Some(object.related_objs.clone())
                }
            }
            ObjectType::Enum(enum_) => None,
        }
    }
}


#[derive(Debug)]
pub struct ConvertOpenApiToRust {
    pub open_api: openapiv3::OpenAPI,
    pub objects: HashMap<String, ObjectType>,
    pub endpoints: Vec<Endpoint>,
    pub plugins: HashMap<String, Plugin>,
}


impl ConvertOpenApiToRust {
    pub fn new(open_api: openapiv3::OpenAPI) -> ConvertOpenApiToRust {
        ConvertOpenApiToRust {
            open_api,
            objects: HashMap::new(),
            endpoints: vec![],
            plugins: HashMap::new(),
        }
    }

    pub fn generate_all(&self) -> Result<(), std::io::Error> {
        for (_, plugin) in &self.plugins {
            plugin.generate()
        }
        let mut all_plugins = self.plugins.iter().map(|(_, plugin)| plugin.name.clone()).collect::<Vec<String>>();
        all_plugins.push("common".to_string());
        all_plugins.push("lol_api_generator".to_string());
        all_plugins.push("lol_client".to_string());
        let crates_path=  PathBuf::from("crates");
        // list dir in crates_path
        let mut crates = std::fs::read_dir(&crates_path).unwrap().map(|res| res.map(|e| e.path())).collect::<Result<Vec<_>, std::io::Error>>().unwrap();
        // make diff
        let mut diff = crates.clone();
        diff.retain(|x| !all_plugins.contains(&x.file_name().unwrap().to_str().unwrap().to_string()));
        if diff.len() > 0 {
            println!("Some plugin remains in the crates folder");
            for path in &diff {
                println!("    {}", path.display());
            }
            // check if the user wants to remove the remaining plugins
            let mut input = String::new();
            print!("Do you want to remove the remaining plugins? [y/N]: ");
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut input).unwrap();
            if input.trim().to_lowercase() == "y" {
                for path in &diff {
                    std::fs::remove_dir_all(path).unwrap();
                }
            }

        }
        Ok(())
    }

    pub fn parse_all(&mut self) -> Result<(), String> {
        self.parse_objects_enums()?;
        self.parse_endpoints()?;
        for endpoint in &self.endpoints {
            let entry = self.plugins.entry(endpoint.plugin.clone()).or_insert(Plugin::new(endpoint.plugin.clone()));
            entry.endpoints.push(endpoint.clone());
        }
        for plugin in self.plugins.values_mut() {
            for endpoint in plugin.endpoints.clone() {
                let mut all_import_done = HashSet::new();
                let mut all_import_todo = endpoint.related_objs.clone();
                while !all_import_todo.is_empty() {
                    let selected_import = all_import_todo.iter().next().unwrap().clone();
                    all_import_todo.remove(&selected_import);
                    if all_import_done.contains(&selected_import) {
                        continue;
                    }
                    let object_ = self.objects.get(&selected_import).cloned().unwrap();
                    if let Some(imports) = object_.get_imports() {
                        all_import_todo.extend(imports);
                    }
                    all_import_done.insert(selected_import);
                }
                for import in all_import_done {
                    plugin.add_object(self.objects.get(&import).cloned().unwrap());
                }
            }
        }
        let mut plugins_len = self.plugins.len();
        let mut i = 0;
        while i < plugins_len {
            let (mut key1, mut plugin1) = {
                let (mut key1, mut plugin1) = self.plugins.iter().nth(i).unwrap();
                (key1.clone(), plugin1.clone())
            };
            for j in 0..plugins_len {
                if i != j {
                    let (mut key2, mut plugin2) = {
                        let (key2,  plugin2) = self.plugins.iter().nth(j).unwrap();
                        (key2.clone(), plugin2.clone())
                    };

                    let plugin1_imports = plugin1.objects.keys().cloned().collect::<HashSet<String>>();
                    let plugin2_imports = plugin2.objects.keys().cloned().collect::<HashSet<String>>();
                    let common_imports = plugin1_imports.intersection(&plugin2_imports).collect::<HashSet<&String>>();
                    if !common_imports.is_empty() {
                        if key2 < key1{
                            let temp_key = key2.clone();
                            key2 = key1.clone();
                            key1 = temp_key;
                        }
                        let old_plugin = self.plugins.remove(&key2).unwrap();
                        let mut new_plugin = self.plugins.remove(&key1).unwrap();
                        let new_key = format!("{}_{}", key1, key2.replace("plugin_lol_", ""));
                        println!("Merging {} and {} => {}",& key1, &key2, &new_key);
                        new_plugin.name = new_key;
                        for endpoint in old_plugin.endpoints {
                            new_plugin.endpoints.push(endpoint);
                        }
                        for object in old_plugin.objects.values() {
                            new_plugin.add_object(object.clone());
                        }
                        self.plugins.insert(new_plugin.name.clone(), new_plugin);
                        plugins_len = self.plugins.len();
                        i = 0;
                        break;
                    }
                }
            }
            i += 1;
        }
        Ok(())
    }


    pub fn get_result_path() -> std::path::PathBuf {
        std::path::PathBuf::from("lol_apis").join("src")
    }



    pub fn parse_endpoints(&mut self) -> Result<(), String> {
        for (path, endpoints) in self.open_api.paths.iter() {
            if let ReferenceOr::Item(path_item) = endpoints {
                if let Some(operation) = &path_item.get {
                    let (parameters, queries) = utils::parse_parameters_queries(&operation.parameters);
                    self.endpoints.push(Endpoint::new(
                        operation.operation_id.clone(),
                        operation.tags.clone(),
                        path.clone(),
                        reqwest::Method::GET,
                        operation.description.clone(),
                        parameters,
                        queries,
                        utils::parse_response(&operation.responses),
                        utils::parse_body(&operation.request_body),
                    ));
                }
                if let Some(operation) = &path_item.post {
                    let (parameters, queries) = utils::parse_parameters_queries(&operation.parameters);
                    self.endpoints.push(Endpoint::new(
                        operation.operation_id.clone(),
                        operation.tags.clone(),
                        path.clone(),
                        reqwest::Method::POST,
                        operation.description.clone(),
                        parameters,
                        queries,
                        utils::parse_response(&operation.responses),
                        utils::parse_body(&operation.request_body),
                    ));
                }
                if let Some(operation) = &path_item.put {
                    let (parameters, queries) = utils::parse_parameters_queries(&operation.parameters);
                    self.endpoints.push(Endpoint::new(
                        operation.operation_id.clone(),
                        operation.tags.clone(),
                        path.clone(),
                        reqwest::Method::PUT,
                        operation.description.clone(),
                        parameters,
                        queries,
                        utils::parse_response(&operation.responses),
                        utils::parse_body(&operation.request_body),
                    ));
                }
                if let Some(operation) = &path_item.patch {
                    let (parameters, queries) = utils::parse_parameters_queries(&operation.parameters);
                    self.endpoints.push(Endpoint::new(
                        operation.operation_id.clone(),
                        operation.tags.clone(),
                        path.clone(),
                        reqwest::Method::PATCH,
                        operation.description.clone(),
                        parameters,
                        queries,
                        utils::parse_response(&operation.responses),
                        utils::parse_body(&operation.request_body),
                    ));
                }
                if let Some(operation) = &path_item.delete {
                    let (parameters, queries) = utils::parse_parameters_queries(&operation.parameters);
                    self.endpoints.push(Endpoint::new(
                        operation.operation_id.clone(),
                        operation.tags.clone(),
                        path.clone(),
                        reqwest::Method::DELETE,
                        operation.description.clone(),
                        parameters,
                        queries,
                        utils::parse_response(&operation.responses),
                        utils::parse_body(&operation.request_body),
                    ));
                }

                if let Some(operation) = &path_item.head {
                    let (parameters, queries) = utils::parse_parameters_queries(&operation.parameters);
                    self.endpoints.push(Endpoint::new(
                        operation.operation_id.clone(),
                        operation.tags.clone(),
                        path.clone(),
                        reqwest::Method::HEAD,
                        operation.description.clone(),
                        parameters,
                        queries,
                        utils::parse_response(&operation.responses),
                        utils::parse_body(&operation.request_body),
                    ));
                }

                if let Some(operation) = &path_item.options {
                    println!("OPTIONS not supported");
                }

                if let Some(operation) = &path_item.trace {
                    println!("TRACE not supported");
                }
            } else {
                println!("ReferenceOr::Reference not supported");
            }
        }
        Ok(())
    }


    pub fn parse_objects_enums(&mut self) -> Result<(), String> {
        if let Some(components) = &self.open_api.components {
            for (name, schema) in components.schemas.iter() {
                let name = utils::format_struct_name(name);
                if let ReferenceOr::Item(schema) = schema {
                    match &schema.schema_kind {
                        SchemaKind::Type(type_) => {
                            if let Type::Object(object) = type_ {
                                let description = schema.schema_data.description.clone().unwrap_or("".to_string());
                                //println!("{}", name);
                                let fields = object.properties.iter().filter_map(|(field_name, field)| {
                                    let required = object.required.contains(field_name);
                                    match field {
                                        ReferenceOr::Reference { reference } => {
                                            Some(Parameter::new(field_name.clone(), required, RustType::Object(utils::parse_reference(reference.as_str()))))
                                        }
                                        ReferenceOr::Item(item) => {
                                            if let SchemaKind::Type(e) = item.schema_kind.clone() {
                                                let type_: RustType = e.clone().into();
                                                Some(Parameter::new(field_name.clone(), required, type_))
                                            } else {
                                                println!("{} => {:?}", name, item);
                                                panic!("SchemaKind not supported");
                                            }
                                        }
                                    }
                                }).collect::<Vec<Parameter>>();
                                let object_ = Object::new(name.clone(), description, fields);
                                self.objects.insert(
                                    name.clone(),
                                    ObjectType::Object(object_),
                                );
                            };
                        }
                        SchemaKind::Any(any_schema) => {
                            if !any_schema.enumeration.is_empty() {
                                let mut variants = vec![];
                                for variant in any_schema.enumeration.iter() {
                                    if let Value::String(variant) = variant {
                                        variants.push(variant.clone());
                                    }
                                }
                                let enum_ = Enum::new(name.clone(), variants);
                                self.objects.insert(
                                    name.clone(),
                                    ObjectType::Enum(enum_),
                                );
                            } else {
                                panic!("{} => {:?}", name, any_schema);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }
}