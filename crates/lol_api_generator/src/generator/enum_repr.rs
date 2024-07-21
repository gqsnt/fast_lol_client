use std::collections::{HashMap, HashSet};
use std::fmt;
use convert_case::{Case, Casing};

pub const ENUM_TEMPLATE: &str = r#"
#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum {enum_name} {
{variants}}
"#;


#[derive(Debug, Clone)]
pub struct Enum {
    pub name:String,
    pub variants: Vec<EnumVariant>,
}

#[derive(Debug, Clone)]
pub struct EnumVariant {
    pub name: String,
    pub rename:Option<String>,
}


impl fmt::Display for EnumVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(rename) = &self.rename {
            write!(f, "    #[serde(rename = \"{}\")]\n", self.name)?;
            write!(f, "    {},\n", rename)
        }else{
            write!(f, "    {},\n", self.name)
        }
    }
}

impl Enum{
    pub fn new(name: String, variants: Vec<String>) -> Enum {
        Enum {
            name,
            variants:variants.iter().map(|variant| {
                let rename = variant.replace("-", "_").to_case(Case::Pascal);
                EnumVariant {
                    name: variant.clone(),
                    rename: if rename != *variant { Some(rename) } else { None },
                }
            }).collect(),
        }
    }

}

impl fmt::Display for Enum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let enum_variants = self.variants.iter().enumerate().map(|(i, variant)| {
            format!("{}{}", if i == 0 { "    #[default]\n" } else { "" }, variant)
        }).collect::<Vec<String>>().join("");
        ENUM_TEMPLATE
            .replace("{enum_name}", &self.name)
            .replace("{variants}", &enum_variants)
            .fmt(f)
    }
}
