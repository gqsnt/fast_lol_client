use serde::Serialize;

pub trait ApiRequest {
    const METHOD: reqwest::Method;
    type ReturnType: serde::de::DeserializeOwned+ Serialize;
    const PLUGIN: crate::client::plugin::LolApiPlugin;
    fn get_path(&self) -> String;
    fn get_body(&self) -> Option<serde_json::Value>;

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
        pub struct $plugin_name;

        impl $plugin_name {
            $(
                impl_api_plugin!(@method_impl
                    $method_name,
                    $endpoint,
                    $( $( $param_name: $param_type, )* )?
                    $( body: $body_type, )?
                );
            )*
        }

        $(
            impl_api_plugin!(@endpoint_struct
                $endpoint,
                $( $( $param_name: $param_type, )* )?
                $( body: $body_type, )?
            );



            impl ApiRequest for $endpoint {
                const METHOD: reqwest::Method = $method;
                type ReturnType = $return_type;
                const PLUGIN: crate::client::plugin::LolApiPlugin = crate::client::plugin::LolApiPlugin::$plugin_name;

                fn get_path(&self) -> String {
                    let mut path = $url.to_string();
                    $(
                        $(
                            path = path.replace(&format!("{{{}}}", stringify!($param_name)), &self.$param_name.to_string());
                        )*
                    )?
                    path
                }

                impl_api_plugin!(@get_body $(body: $body_type,)?);
            }
        )*
    };


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


    (@endpoint_struct
        $endpoint:ident,
        $($param_name:ident: $param_type:ty,)*
    ) => {
        pub struct $endpoint {
            $($param_name: $param_type),*
        }
    };

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



    (@get_body
        body: $body_type:ty,
    ) => {
        fn get_body(&self) -> Option<serde_json::Value> {
            Some(serde_json::to_value( & self.body).expect("Failed to serialize body"))
        }
    };

    (@get_body
    ) => {
        fn get_body(&self) -> Option<serde_json::Value> {
            None
        }
    };

}