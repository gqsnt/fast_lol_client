use convert_case::{Case, Casing};
use std::fmt;
use crate::generator::rust_type::RustType;

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub rename: Option<String>,
    pub required: bool,
    pub type_: RustType,
}

impl Parameter{

    pub fn get_name(&self) -> &str {
        if let Some(rename) = &self.rename {
            rename
        }else{
            &self.name
        }
    }

    pub fn new(name: String, required: bool, type_: RustType) -> Parameter {
        let mut rename = name.clone().replace("-", "_").to_case(Case::Snake);
        if crate::generator::utils::is_rust_keyword(&rename){
            rename = rename
                .replace("+", "")
                .replace("-", "")
                .replace(" ", "")
                .replace("?", "")
                .replace("!", "")
                .replace(":", "")
                .replace(";", "")
                .replace("=", "")
                .replace(">", "")
                .replace("<", "")
                .replace(",", "")
                .replace(".", "")
                .replace("/", "")
                .replace("\\", "")
                .replace("|", "")
                .replace("(", "")
                .replace(")", "")
                .replace("[", "")
                .replace("]", "")
                .replace("{", "")
                .replace("}", "");

            rename = format!("{}_", rename);
        }
        let rename = if rename != name { Some(rename) } else { None };
        Parameter {
            name,
            rename,
            required,
            type_,
        }
    }

    pub fn to_string_with_rename(&self) -> String{
        if let Some(rename) = &self.rename {
            format!("    #[serde(rename = \"{}\")]\n    pub {}: {},\n", self.name, rename, self.type_)
        }else{
            format!("    pub {}: {},\n", self.get_name(), self.type_)
        }
    }

    pub fn to_string(&self) -> String{
        format!("    pub {}: {},\n", self.get_name(), self.type_)
    }

}