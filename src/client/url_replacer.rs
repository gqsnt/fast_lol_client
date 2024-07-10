use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

pub trait IsReplaceText {
    fn replace(&self, template: String) -> String;
}


macro_rules! define_url_replacer {
    ($struct_name:ident, $($field_name:ident: $field_type:ty),*) => {
        pub struct $struct_name {

            $( $field_name: $field_type ),*
        }

        impl $struct_name {
            fn new($($field_name: $field_type),*) -> Self {
                Self { $( $field_name ),* }
            }
        }

        impl IsReplaceText for $struct_name {
            fn replace(&self, template: String) -> String {
                let mut url = template;
                // Iterate over each field to replace its placeholder in the URL template
                $(
                    url = url.replace(&format!("{{{}}}", stringify!($field_name)), &self.$field_name.to_string());
                )*
                url
            }
        }
    };
}

define_url_replacer!(Id, id: u32);
