use std::collections::HashSet;
use std::fmt;
use crate::generator::param::Parameter;
use crate::generator::rust_type::RustType;
use crate::generator::utils::{get_object_name_in_type};


pub const OBJECT_TEMPLATE: &str = r#"
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct {object_name} {{fields}}
"#;



#[derive(Debug, Clone)]
pub struct Object {
    pub description: String,
    pub name: String,
    pub fields: Vec<Parameter>,
    pub related_objs:HashSet<String>
}

impl Object{
    pub fn new(name: String,description:String, fields:Vec<Parameter>) -> Object {
        let mut related_objs = HashSet::new();
        for field in &fields{
            if let Some(object_name) = get_object_name_in_type(&field.type_){
                related_objs.insert(object_name);
            }
        }

        Object {
            description,
            name,
            related_objs,
            fields
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut attributes = self.fields.iter().map(|field| {
            field.to_string_with_rename()
        }).collect::<Vec<String>>().join("\n");
        if !attributes.is_empty(){
            attributes = format!("\n{}\n", attributes);
        }
        OBJECT_TEMPLATE
            .replace("{object_name}", &self.name)
            .replace("{fields}", &attributes)
            .fmt(f)
    }
}