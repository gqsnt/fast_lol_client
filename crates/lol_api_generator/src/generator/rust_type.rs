use std::fmt;

use openapiv3::{AdditionalProperties, IntegerFormat, IntegerType, NumberFormat, NumberType, ObjectType, ReferenceOr, SchemaKind, VariantOrUnknownOrEmpty};

#[derive(Debug, Clone, PartialEq)]
pub enum RustType {
    String,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Float32,
    Float64,
    Boolean,
    Object(Option<String>),
    HashMap(Box<RustType>),
    Array(Box<RustType>),
    Int16,
    Int8,
}


impl fmt::Display for RustType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match &self {
            RustType::String => "String".to_string(),
            RustType::Int32 => "i32".to_string(),
            RustType::Int64 => "i64".to_string(),
            RustType::UInt8 => "u8".to_string(),
            RustType::UInt16 => "u16".to_string(),
            RustType::UInt32 => "u32".to_string(),
            RustType::UInt64 => "u64".to_string(),
            RustType::Float32 => "f32".to_string(),
            RustType::Float64 => "f64".to_string(),
            RustType::Boolean => "bool".to_string(),
            RustType::Object(name) => {
                name.clone().unwrap_or_else(|| "Value".to_string())
            }
            RustType::HashMap(inner) => format!("HashMap<String, {}>", inner),
            RustType::Array(inner) => format!("Vec<{}>", inner),
            RustType::Int16 => "i16".to_string(),
            RustType::Int8 => "i8".to_string(),
        })
    }
}


impl From<NumberType> for RustType {
    fn from(number_type: NumberType) -> Self {
        match number_type.format {
            VariantOrUnknownOrEmpty::Item(NumberFormat::Float) => RustType::Float32,
            VariantOrUnknownOrEmpty::Item(NumberFormat::Double) => RustType::Float64,
            _ => RustType::Float64,
        }
    }
}

impl From<IntegerType> for RustType {
    fn from(integer_format: IntegerType) -> Self {
        match integer_format.format {
            VariantOrUnknownOrEmpty::Item(IntegerFormat::Int32) => RustType::Int32,
            VariantOrUnknownOrEmpty::Item(IntegerFormat::Int64) => RustType::Int64,
            VariantOrUnknownOrEmpty::Unknown(name) => {
                if name == "uint8" { RustType::UInt8 } else if name == "uint16" { RustType::UInt16 } else if name == "uint32" { RustType::UInt32 } else if name == "uint64" { RustType::UInt64 } else if name == "int16" { RustType::Int16 } else if name == "int8" { RustType::Int8 } else {
                    panic!("Unknown integer format: {}", name);
                }
            }
            VariantOrUnknownOrEmpty::Empty => RustType::Int64,
        }
    }
}

impl From<openapiv3::Type> for RustType {
    fn from(type_: openapiv3::Type) -> Self {
        match type_ {
            openapiv3::Type::String(string) => RustType::String,
            openapiv3::Type::Number(number) => RustType::from(number),
            openapiv3::Type::Integer(integer) => RustType::from(integer),
            openapiv3::Type::Object(object) => match object {
                ObjectType { additional_properties, .. } => {
                    if let Some(additional_properties) = additional_properties {
                        match additional_properties {
                            AdditionalProperties::Schema(schema) => {
                                match schema.as_ref() {
                                    ReferenceOr::Reference { reference } => {
                                        RustType::Object(crate::generator::utils::parse_reference(reference.as_str()))
                                    }
                                    ReferenceOr::Item(item) => {
                                        match item.schema_kind.clone() {
                                            SchemaKind::Type(type_) => {
                                                RustType::HashMap(Box::new(type_.into()))
                                            }
                                            _ => {
                                                panic!("SchemaKind not supported");
                                            }
                                        }
                                    }
                                }
                            }
                            AdditionalProperties::Any(any) => {
                                RustType::HashMap(Box::new(RustType::String))
                            }
                        }
                    } else {
                        panic!("AdditionalProperties not supported");
                    }
                }
            },
            openapiv3::Type::Array(array) => RustType::Array(Box::new(crate::generator::utils::get_child_schema(array))),
            openapiv3::Type::Boolean(_) => { RustType::Boolean }
        }
    }
}