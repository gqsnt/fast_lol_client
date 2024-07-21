
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

// ENDPOINTS

pub struct GetByPluginAssetsByPath {
    // Download a backend asset
    pub plugin: String,
    pub path_: String,
    pub if_none_match: String,
}

impl IsApiRequest for GetByPluginAssetsByPath {
    const METHOD: Method = Method::GET;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        format!("/{}/assets/{}", self.plugin, self.path_)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "if-none-match" : self.if_none_match,
        }))
    }
}

pub fn get_by_plugin_assets_by_path(plugin: String, path_: String, if_none_match: String) -> GetByPluginAssetsByPath {
    GetByPluginAssetsByPath {
        plugin, path_, if_none_match
    }
}


pub struct HeadByPluginAssetsByPath {
    // Download the header for a backend asset
    pub plugin: String,
    pub path_: String,
    pub if_none_match: String,
}

impl IsApiRequest for HeadByPluginAssetsByPath {
    const METHOD: Method = Method::HEAD;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        format!("/{}/assets/{}", self.plugin, self.path_)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        Some(json!({
            "if-none-match" : self.if_none_match,
        }))
    }
}

pub fn head_by_plugin_assets_by_path(plugin: String, path_: String, if_none_match: String) -> HeadByPluginAssetsByPath {
    HeadByPluginAssetsByPath {
        plugin, path_, if_none_match
    }
}


pub struct GetPluginManagerV1ExternalPluginsAvailability {
    // Get the status of the external plugin connection.
}

impl IsApiRequest for GetPluginManagerV1ExternalPluginsAvailability {
    const METHOD: Method = Method::GET;
    type ReturnType = ExternalPluginsResource;

    fn get_url(&self) -> String {
        "/plugin-manager/v1/external-plugins/availability".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_plugin_manager_v_1_external_plugins_availability() -> GetPluginManagerV1ExternalPluginsAvailability {
    GetPluginManagerV1ExternalPluginsAvailability {
        
    }
}


pub struct GetPluginManagerV1Status {
    // Get the status of the plugin manager.
}

impl IsApiRequest for GetPluginManagerV1Status {
    const METHOD: Method = Method::GET;
    type ReturnType = PluginManagerResource;

    fn get_url(&self) -> String {
        "/plugin-manager/v1/status".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_plugin_manager_v_1_status() -> GetPluginManagerV1Status {
    GetPluginManagerV1Status {
        
    }
}


pub struct GetPluginManagerV2Descriptions {
    // Get all plugin descriptions.
}

impl IsApiRequest for GetPluginManagerV2Descriptions {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<PluginDescriptionResource>;

    fn get_url(&self) -> String {
        "/plugin-manager/v2/descriptions".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_plugin_manager_v_2_descriptions() -> GetPluginManagerV2Descriptions {
    GetPluginManagerV2Descriptions {
        
    }
}


pub struct GetPluginManagerV2DescriptionsByPlugin {
    // Get a plugin description.
    pub plugin: String,
}

impl IsApiRequest for GetPluginManagerV2DescriptionsByPlugin {
    const METHOD: Method = Method::GET;
    type ReturnType = PluginDescriptionResource;

    fn get_url(&self) -> String {
        format!("/plugin-manager/v2/descriptions/{}", self.plugin)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_plugin_manager_v_2_descriptions_by_plugin(plugin: String) -> GetPluginManagerV2DescriptionsByPlugin {
    GetPluginManagerV2DescriptionsByPlugin {
        plugin
    }
}


pub struct GetPluginManagerV2Plugins {
    // Get diagnostic information for all plugins.
}

impl IsApiRequest for GetPluginManagerV2Plugins {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<PluginResource>;

    fn get_url(&self) -> String {
        "/plugin-manager/v2/plugins".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_plugin_manager_v_2_plugins() -> GetPluginManagerV2Plugins {
    GetPluginManagerV2Plugins {
        
    }
}


pub struct GetPluginManagerV2PluginsByPlugin {
    // Get diagnostic information for a single plugin.
    pub plugin: String,
}

impl IsApiRequest for GetPluginManagerV2PluginsByPlugin {
    const METHOD: Method = Method::GET;
    type ReturnType = PluginResource;

    fn get_url(&self) -> String {
        format!("/plugin-manager/v2/plugins/{}", self.plugin)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_plugin_manager_v_2_plugins_by_plugin(plugin: String) -> GetPluginManagerV2PluginsByPlugin {
    GetPluginManagerV2PluginsByPlugin {
        plugin
    }
}


pub struct GetPluginManagerV3PluginsManifest {
    // Get the plugin manifest.
}

impl IsApiRequest for GetPluginManagerV3PluginsManifest {
    const METHOD: Method = Method::GET;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/plugin-manager/v3/plugins-manifest".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_plugin_manager_v_3_plugins_manifest() -> GetPluginManagerV3PluginsManifest {
    GetPluginManagerV3PluginsManifest {
        
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PluginManagerResource {
    pub state: PluginManagerState,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PluginDescriptionResource {
    pub name: String,
    pub riot_meta: PluginMetadataResource,
    pub plugin_dependencies: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PluginResource {
    pub full_name: String,
    pub short_name: String,
    pub supertype: String,
    pub subtype: String,
    pub app: String,
    pub feature: String,
    pub threading_model: String,
    pub asset_bundle_names: Vec<String>,
    pub mounted_asset_bundles: HashMap<String, String>,
    pub order_wad_file_mounted: i32,
    pub dependencies: Vec<PluginResourceContract>,
    pub implemented_contracts: Vec<PluginResourceContract>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExternalPluginsResource {
    pub state: ExternalPluginsAvailability,
    pub error_string: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PluginMetadataResource {
    pub type_: String,
    pub subtype: String,
    pub app: String,
    pub feature: String,
    pub mock: String,
    pub has_bundled_assets: bool,
    pub global_asset_bundles: Vec<String>,
    pub per_locale_asset_bundles: HashMap<String, HashMap<String, String>>,
    pub implements: Vec<String>,
    pub threading: PluginThreadingModel,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PluginResourceContract {
    pub full_name: String,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum PluginManagerState {
    #[default]
    PluginsInitialized,
    NotReady,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum PluginThreadingModel {
    #[default]
    #[serde(rename = "parallel")]
    Parallel,
    #[serde(rename = "concurrent")]
    Concurrent,
    #[serde(rename = "sequential")]
    Sequential,
    #[serde(rename = "dedicated")]
    Dedicated,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum ExternalPluginsAvailability {
    #[default]
    Error,
    Recovering,
    Connected,
    Preparing,
    NotAvailable,
}

