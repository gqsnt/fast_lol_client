use serde::Serialize;

pub trait ApiRequest {
    const METHOD: reqwest::Method;
    type ReturnType: serde::de::DeserializeOwned + Serialize;
    const PLUGIN: crate::client::plugin::LolApiPlugin;
    const ENDPOINT: &'static str;
    fn get_path(&self) -> String{
        Self::ENDPOINT.to_string()
    }
    fn get_body(&self) -> Option<serde_json::Value> {
        None
    }

    fn get_url(&self) -> String {
        format!("{}{}", Self::PLUGIN.get_path(), self.get_path())
    }

}


#[macro_export]
macro_rules! impl_api_plugin {
    (
        $plugin_name:ident,
        $(
            $endpoint:ident {
                $method_name:ident,
                $method:expr,
                $url:expr =>
                $return_type:ty,
                $(route_params: {$($param_name:ident: $param_type:ty),* $(,)*},)?
                $(body: $body_type:ty,)?
            },
        )*
    ) => {
        use crate::client::request::ApiRequest;

        // Struct representing the plugin
        pub struct $plugin_name;

        impl $plugin_name {
            $(
                // Method implementation for each endpoint
                impl_api_plugin!(@method_impl
                    $method_name,
                    $endpoint,
                    $( $( $param_name: $param_type, )* )?
                    $( body: $body_type, )?
                );
            )*
        }

        $(
            // Struct representing each endpoint
            impl_api_plugin!(@endpoint_struct
                $endpoint,
                $( $( $param_name: $param_type, )* )?
                $( body: $body_type, )?
            );


            // Implementing the ApiRequest trait for each endpoint
            impl ApiRequest for $endpoint {
                const METHOD: reqwest::Method = $method;
                type ReturnType = $return_type;
                const PLUGIN: crate::client::plugin::LolApiPlugin = crate::client::plugin::LolApiPlugin::$plugin_name;
                const ENDPOINT: &'static str = $url;


                // Method to get the URL path with parameters replaced
                impl_api_plugin!(@get_path
                    $(params: $( $param_name: $param_type, )*)?
                );

                // Method to get the request body as JSON
                impl_api_plugin!(@get_body $(body: $body_type,)?);
            }
        )*
    };


    // Helper rule to implement the method for the plugin
    (@method_impl
        $method_name:ident,
        $endpoint:ident,
        $($param_name:ident: $param_type:ty,)*
    ) => {
        pub fn $method_name($($param_name: $param_type),*) -> $endpoint {
            $endpoint {
                $($param_name),*
            }
        }
    };


    // Helper rule to implement the method with body for the plugin
    (@method_impl
        $method_name:ident,
        $endpoint:ident,
        $($param_name:ident: $param_type:ty,)*
        body: $body_type:ty,
    ) => {
        pub fn $method_name($($param_name: $param_type),*, body: $body_type) -> $endpoint {
            $endpoint {
                $($param_name),*,
                body,
            }
        }
    };


    // Helper rule to define the struct for the endpoint
    (@endpoint_struct
        $endpoint:ident,
        $($param_name:ident: $param_type:ty,)*
    ) => {
        pub struct $endpoint {
            $($param_name: $param_type),*
        }
    };

    // Helper rule to define the struct with body for the endpoint
    (@endpoint_struct
        $endpoint:ident,
        $($param_name:ident: $param_type:ty,)*
        body: $body_type:ty,
    ) => {
        pub struct $endpoint {
            $($param_name: $param_type),*,
            body: $body_type,
        }
    };



    // Helper rule to implement the get_body method
    (@get_body
        body: $body_type:ty,
    ) => {
        fn get_body(&self) -> Option<serde_json::Value> {
            Some(serde_json::to_value( & self.body).expect("Failed to serialize body"))
        }
    };

    // Helper rule to implement the get_path method for endpoints with parameters
    (@get_path
        $(params: $($param_name:ident: $param_type:ty,)*)?
    ) => {
        $(
            fn get_path(&self) -> String{
                let mut path = Self::ENDPOINT.to_string();
                $(
                    path = path.replace(&format!("{{{}}}", stringify!($param_name)), &self.$param_name.to_string());
                )*
                path
            }
        )?

    };

    // Default implementations for endpoints without body or parameters
    (@get_body) => {};
    (@get_path) => {};
}
pub use impl_api_plugin;


//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::client::plugin::LolApiPlugin;
//     use crate::client::request::ApiRequest;
//     use reqwest::Method;
//     use serde::Deserialize;
//     use serde_json::Value;
//
//
//
//     impl_api_plugin!(
//         LolChat,
//         TestEndpoint {
//             test_method, Method::GET, "/test/{id}" => Value,
//             route_params: { id: u64 },
//         },
//         TestEndpointWithBody {
//             test_method_with_body, Method::POST, "/test" => Value,
//             body: TestBody,
//         },
//     );
//
//     #[derive(Debug, Clone, Serialize, Deserialize)]
//     pub struct TestBody {
//         pub field: String,
//     }
//
//     #[test]
//     fn test_macro_without_body() {
//         let endpoint = LolChat::test_method(42);
//         assert_eq!(endpoint.get_path(), "/test/42");
//         assert_eq!(TestEndpoint::METHOD, Method::GET);
//         assert_eq!(TestEndpoint::ENDPOINT, "/test/{id}");
//         assert_eq!(TestEndpoint::PLUGIN, LolApiPlugin::LolChat);
//     }
//
//     #[test]
//     fn test_macro_with_body() {
//         let body = TestBody {
//             field: "test".into(),
//         };
//         let endpoint = LolChat::test_method_with_body(body.clone());
//         assert_eq!(endpoint.get_path(), "/test");
//         assert_eq!(endpoint.get_body(), Some(serde_json::to_value(&body).unwrap()));
//         assert_eq!(TestEndpointWithBody::METHOD, Method::POST);
//         assert_eq!(TestEndpointWithBody::ENDPOINT, "/test");
//         assert_eq!(TestEndpointWithBody::PLUGIN, LolApiPlugin::LolChat);
//     }
// }