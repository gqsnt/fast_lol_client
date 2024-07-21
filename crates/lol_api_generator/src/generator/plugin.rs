use std::collections::HashMap;
use std::path::PathBuf;
use crate::generator::endpoint::Endpoint;
use crate::generator::enum_repr::Enum;
use crate::generator::object_repr::Object;
use crate::generator::ObjectType;

pub const PLUGIN_CARGO_TEMPLATE: &str = r#"
[package]
name = "{name}"
version = "0.1.0"
edition = "2021"

[dependencies]
reqwest ="0.12"
serde_json = "1.0"
serde = { version = "1.0", features = ["derive"] }
common = { path = "../common" }
"#;


pub const PLUGIN_LIB_TEMPLATE: &str = r#"
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

// ENDPOINTS
{endpoints}

// OBJECTS
{objects}

// ENUMS
{enums}
"#;


#[derive(Debug, Clone)]
pub struct Plugin{
    pub name: String,
    pub imports: HashMap<String, Vec<String>>,
    pub endpoints: Vec<Endpoint>,
    pub objects: HashMap<String, ObjectType>,
}

impl Plugin{
    pub fn new(name: String) -> Plugin {
        let mut name = name;
        if !name.contains("plugin_lol") {
            name = format!("plugin_lol_{}", name);
        }
        Plugin {
            name,
            imports: HashMap::new(),
            endpoints: Vec::new(),
            objects: HashMap::new(),
        }
    }

    pub fn add_import(&mut self, plugin: String, import: String) {
        self.imports.entry(plugin).or_insert_with(Vec::new).push(import);
    }

    pub fn add_object(&mut self, object: ObjectType) {
        self.objects.insert(object.get_inner_name().to_string(), object);
    }



    pub fn generate(&self) {
        let cargo = PLUGIN_CARGO_TEMPLATE.replace("{name}", &self.name);
        let lib = PLUGIN_LIB_TEMPLATE;
        let plugin_path = PathBuf::from("crates").join(&self.name);
        let cargo_path = plugin_path.join("Cargo.toml");
        let src_path = plugin_path.join("src");
        let lib_path = src_path.join("lib.rs");

        if !plugin_path.exists() {
            let _ = std::fs::create_dir_all(&plugin_path);
            let _ = std::fs::create_dir_all(&src_path);
            let _ = std::fs::write(&cargo_path, &cargo);
            let _ = std::fs::write(&lib_path, &lib);
        }

        let endpoints = self.endpoints.iter().map(|endpoint| {
            endpoint.to_string()
        }).collect::<Vec<String>>().join("\n");



        let objects = self.objects.iter().filter_map(|(_, object)| {
            match object {
                ObjectType::Object(object_) => Some(object_.to_string()),
                ObjectType::Enum(_) => None,
            }
        }).collect::<Vec<String>>().join("\n");

        let enums = self.objects.iter().filter_map(|(_, object)| {
            match object {
                ObjectType::Object(_) => None,
                ObjectType::Enum(enum_) => Some(enum_.to_string()),
            }
        }).collect::<Vec<String>>().join("\n");

        let lib = lib
            .replace("{endpoints}", &endpoints)
            .replace("{objects}", &objects)
            .replace("{enums}", &enums);
        let _ = std::fs::write(&lib_path, &lib);

    }
}