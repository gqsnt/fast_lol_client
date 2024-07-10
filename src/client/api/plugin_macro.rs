

#[macro_export]
macro_rules! impl_api_plugin {
    (
        $plugin:ident,
        $(
            $endpoint:ident {
                $fn_name:ident,
                $method:expr,
                $url:expr =>
                $return_type:ty,
                $(route_params: {$($param_name:ident: $param_type:ty),* $(,)*},)?
                $(body: $body_type:ty,)?
            },
        )*
    ) => {
        use crate::client::api::is_api_request::IsApiRequest;
        use crate::client::api::plugin::LolApiPlugin;





        $(
            // Struct representing each endpoint
            impl_api_plugin!(@endpoint_struct
                $endpoint,
                $( $( $param_name: $param_type, )* )?
                $( body: $body_type, )?
            );

            // Helper function to create the endpoint struct
            impl_api_plugin!(@helper_fn_impl
                $fn_name,
                $endpoint,
                $( $( $param_name: $param_type, )* )?
                $( body: $body_type, )?
            );


            // Implementing the ApiRequest trait for each endpoint
            impl IsApiRequest for $endpoint {
                const METHOD: reqwest::Method = $method;
                type ReturnType = $return_type;
                const PLUGIN: LolApiPlugin = LolApiPlugin::$plugin;
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


    // Helper rule to implement the function for the plugin
    (@helper_fn_impl
        $fn_name:ident,
        $endpoint:ident,
        $($param_name:ident: $param_type:ty,)*
    ) => {
        pub fn $fn_name($($param_name: $param_type),*) -> $endpoint {
            $endpoint {
                $($param_name),*
            }
        }
    };


    // Helper rule to implement the function with body for the plugin
    (@helper_fn_impl
        $fn_name:ident,
        $endpoint:ident,
        $($param_name:ident: $param_type:ty,)*
        body: $body_type:ty,
    ) => {
        pub fn $fn_name($($param_name: $param_type),*, body: $body_type) -> $endpoint {
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

