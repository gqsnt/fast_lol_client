use std::collections::HashSet;
use std::fmt;
use crate::generator::param::Parameter;
use crate::generator::rust_type::RustType;
use crate::generator::utils::{crate_path_from_type_and_name, get_object_name_in_type};


pub const OBJECT_TEMPLATE: &str = r#"
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct {object_name} {
{fields}}
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
        let mut attributes = String::new();
        for field in self.fields.iter() {
            attributes.push_str(&format!("{}", field.to_string()));
        }
        OBJECT_TEMPLATE
            .replace("{object_name}", &self.name)
            .replace("{fields}", &attributes)
            .fmt(f)
    }
}