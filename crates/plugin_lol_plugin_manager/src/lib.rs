
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetByPluginAssetsByPath {
    /// Download a backend asset
    pub plugin: String,
    pub path_: String,
    pub if_none_match: Option<String>,
}

impl IsApiRequest for GetByPluginAssetsByPath {
    const METHOD: Method = Method::GET;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/{}/assets/{}", self.plugin, self.path_)}
    fn get_query(&self) -> Option<Vec<(String,String)>> {
        Some(vec![
            ("if-none-match".to_string(), serde_json::to_string(&self.if_none_match).unwrap())
        ])
    }
}

pub fn get_by_plugin_assets_by_path(plugin: String, path_: String, if_none_match: Option<String>) -> GetByPluginAssetsByPath {
    GetByPluginAssetsByPath{plugin, path_, if_none_match}
}


pub struct GetPluginManagerV1ExternalPluginsAvailability {
    /// Get the status of the external plugin connection.

}

impl IsApiRequest for GetPluginManagerV1ExternalPluginsAvailability {
    const METHOD: Method = Method::GET;
    type ReturnType = ExternalPluginsResource;
    fn get_url(&self) -> String {"/plugin-manager/v1/external-plugins/availability".to_string()}
}

pub fn get_plugin_manager_v1_external_plugins_availability() -> GetPluginManagerV1ExternalPluginsAvailability {
    GetPluginManagerV1ExternalPluginsAvailability{}
}


pub struct GetPluginManagerV1Status {
    /// Get the status of the plugin manager.

}

impl IsApiRequest for GetPluginManagerV1Status {
    const METHOD: Method = Method::GET;
    type ReturnType = PluginManagerResource;
    fn get_url(&self) -> String {"/plugin-manager/v1/status".to_string()}
}

pub fn get_plugin_manager_v1_status() -> GetPluginManagerV1Status {
    GetPluginManagerV1Status{}
}


pub struct GetPluginManagerV2Descriptions {
    /// Get all plugin descriptions.

}

impl IsApiRequest for GetPluginManagerV2Descriptions {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<PluginDescriptionResource>;
    fn get_url(&self) -> String {"/plugin-manager/v2/descriptions".to_string()}
}

pub fn get_plugin_manager_v2_descriptions() -> GetPluginManagerV2Descriptions {
    GetPluginManagerV2Descriptions{}
}


pub struct GetPluginManagerV2DescriptionsByPlugin {
    /// Get a plugin description.
    pub plugin: String,
}

impl IsApiRequest for GetPluginManagerV2DescriptionsByPlugin {
    const METHOD: Method = Method::GET;
    type ReturnType = PluginDescriptionResource;
    fn get_url(&self) -> String {format!("/plugin-manager/v2/descriptions/{}", self.plugin)}
}

pub fn get_plugin_manager_v2_descriptions_by_plugin(plugin: String) -> GetPluginManagerV2DescriptionsByPlugin {
    GetPluginManagerV2DescriptionsByPlugin{plugin}
}


pub struct GetPluginManagerV2Plugins {
    /// Get diagnostic information for all plugins.

}

impl IsApiRequest for GetPluginManagerV2Plugins {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<PluginResource>;
    fn get_url(&self) -> String {"/plugin-manager/v2/plugins".to_string()}
}

pub fn get_plugin_manager_v2_plugins() -> GetPluginManagerV2Plugins {
    GetPluginManagerV2Plugins{}
}


pub struct GetPluginManagerV2PluginsByPlugin {
    /// Get diagnostic information for a single plugin.
    pub plugin: String,
}

impl IsApiRequest for GetPluginManagerV2PluginsByPlugin {
    const METHOD: Method = Method::GET;
    type ReturnType = PluginResource;
    fn get_url(&self) -> String {format!("/plugin-manager/v2/plugins/{}", self.plugin)}
}

pub fn get_plugin_manager_v2_plugins_by_plugin(plugin: String) -> GetPluginManagerV2PluginsByPlugin {
    GetPluginManagerV2PluginsByPlugin{plugin}
}


pub struct GetPluginManagerV3PluginsManifest {
    /// Get the plugin manifest.

}

impl IsApiRequest for GetPluginManagerV3PluginsManifest {
    const METHOD: Method = Method::GET;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/plugin-manager/v3/plugins-manifest".to_string()}
}

pub fn get_plugin_manager_v3_plugins_manifest() -> GetPluginManagerV3PluginsManifest {
    GetPluginManagerV3PluginsManifest{}
}


pub struct HeadByPluginAssetsByPath {
    /// Download the header for a backend asset
    pub plugin: String,
    pub path_: String,
    pub if_none_match: Option<String>,
}

impl IsApiRequest for HeadByPluginAssetsByPath {
    const METHOD: Method = Method::HEAD;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/{}/assets/{}", self.plugin, self.path_)}
    fn get_query(&self) -> Option<Vec<(String,String)>> {
        Some(vec![
            ("if-none-match".to_string(), serde_json::to_string(&self.if_none_match).unwrap())
        ])
    }
}

pub fn head_by_plugin_assets_by_path(plugin: String, path_: String, if_none_match: Option<String>) -> HeadByPluginAssetsByPath {
    HeadByPluginAssetsByPath{plugin, path_, if_none_match}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExternalPluginsResource {
    pub state: ExternalPluginsAvailability,
    #[serde(rename = "errorString")]
    pub error_string: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PluginDescriptionResource {
    pub name: String,
    #[serde(rename = "riotMeta")]
    pub riot_meta: PluginMetadataResource,
    #[serde(rename = "pluginDependencies")]
    pub plugin_dependencies: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PluginManagerResource {
    pub state: PluginManagerState,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PluginMetadataResource {
    #[serde(rename = "type")]
    pub type_: String,
    pub subtype: String,
    pub app: String,
    pub feature: String,
    pub mock: String,
    #[serde(rename = "hasBundledAssets")]
    pub has_bundled_assets: bool,
    #[serde(rename = "globalAssetBundles")]
    pub global_asset_bundles: Vec<String>,
    #[serde(rename = "perLocaleAssetBundles")]
    pub per_locale_asset_bundles: HashMap<String, Value>,
    pub implements: Vec<String>,
    pub threading: PluginThreadingModel,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PluginResource {
    #[serde(rename = "fullName")]
    pub full_name: String,
    #[serde(rename = "shortName")]
    pub short_name: String,
    pub supertype: String,
    pub subtype: String,
    pub app: String,
    pub feature: String,
    #[serde(rename = "threadingModel")]
    pub threading_model: String,
    #[serde(rename = "assetBundleNames")]
    pub asset_bundle_names: Vec<String>,
    #[serde(rename = "mountedAssetBundles")]
    pub mounted_asset_bundles: HashMap<String, String>,
    #[serde(rename = "orderWadFileMounted")]
    pub order_wad_file_mounted: i32,
    pub dependencies: Vec<PluginResourceContract>,
    #[serde(rename = "implementedContracts")]
    pub implemented_contracts: Vec<PluginResourceContract>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PluginResourceContract {
    #[serde(rename = "fullName")]
    pub full_name: String,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum ExternalPluginsAvailability {
    #[default]
    Error,
    Recovering,
    Connected,
    Preparing,
    NotAvailable,
}


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

