

#[macro_export]
macro_rules! impl_api_plugin {
    (
        $plugin_url:expr,
        $(
            $version:ident {
                $(
                    $endpoint:ident {
                        $fn_name:ident,
                        $method:expr,
                        $url:expr
                        $(, params: {$($param_name:ident: $param_type:ty),*})?
                        $(, body: $body_type:ty)?
                    }  => $return_type:ty
                )*
            }
        )*
    ) => {
        use crate::client::apis::is_api_request::IsApiRequest;
        use crate::client::apis::ApiVersion;
        use std::marker::PhantomData;

        $(
            $(
                // Define the endpoint struct
                impl_api_plugin!(@define_struct
                    $endpoint,
                    $( $( $param_name: $param_type, )* )?
                    $( body: $body_type, )?
                );

                // Define the helper function to create the endpoint struct
                impl_api_plugin!(@define_helper_fn
                    $fn_name,
                    $endpoint,
                    $( $( $param_name: $param_type, )* )?
                    $( body: $body_type, )?
                );

                // Implement the IsApiRequest trait for the endpoint
                impl IsApiRequest for $endpoint {
                    const METHOD: reqwest::Method = $method;
                    const API_VERSION: crate::client::apis::ApiVersion = ApiVersion::$version;
                    type ReturnType = $return_type;
                    const PLUGIN_URL: &'static str = $plugin_url;
                    const REQUEST_URL: &'static str = $url;

                    // Generate the URL path with parameters replaced
                    impl_api_plugin!(@generate_path $url $(,params: $( $param_name: $param_type, )*)?);

                    // Generate the request body as JSON
                    impl_api_plugin!(@generate_body $(body: $body_type,)?);
                }
            )*
        )*
    };

    // Define the endpoint struct
    (@define_struct
        $endpoint:ident,
        $($param_name:ident: $param_type:ty,)*
    ) => {
        pub struct $endpoint {
            $(pub $param_name: $param_type),*
        }
    };

    (@define_struct
        $endpoint:ident,
        $($param_name:ident: $param_type:ty,)*
        body: $body_type:ty,
    ) => {
        pub struct $endpoint {
            $(pub $param_name: $param_type),*,
            pub body: $body_type,
        }
    };

    // Define the helper function to create the endpoint struct
    (@define_helper_fn
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

    (@define_helper_fn
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

    (@generate_path $url:expr, params:$($param_name:ident: $param_type:ty,)*) => {
        fn get_path(&self) -> String {
            format!($url, $(self.$param_name),*)
        }
    };



    // Generate the request body as JSON
    (@generate_body
        body: $body_type:ty,
    ) => {
        fn get_body(&self) -> Option<serde_json::Value> {
            Some(serde_json::to_value(&self.body).expect("Failed to serialize body"))
        }
    };

    // Default implementations for endpoints without body or parameters
    (@generate_body) => {};
    (@generate_path $url:expr) => {};
}
pub use impl_api_plugin;