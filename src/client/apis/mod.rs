use crate::client::apis::lol_champ_select::LolChampSelect;
use crate::client::apis::lol_chat::LolChat;
use crate::client::apis::lol_game_flow::LolGameFlow;
use crate::client::apis::lol_lobby::LolLobby;
use crate::client::apis::lol_summoner::LolSummoner;

pub mod lol_summoner;
pub mod lol_chat;
pub mod lol_game_flow;
pub mod lol_lobby;
pub mod lol_champ_select;

pub struct API;



#[macro_export]
macro_rules! impl_api_plugin {
    (
        $plugin_name:ident,
        $plugin_helper_name:ident,
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
        use crate::client::apis::API;

        // Struct representing the plugin
        pub struct $plugin_name;

        impl API{
            pub fn $plugin_helper_name() -> $plugin_name{$plugin_name}
        }

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
        pub fn $method_name(self,$($param_name: $param_type),*) -> $endpoint {
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
        pub fn $method_name(self,$($param_name: $param_type),*, body: $body_type) -> $endpoint {
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






