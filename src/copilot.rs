use lapce_plugin::psp_types::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum SetEditorInfo {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetEditorInfoParams {
    pub editor_info: EditorInfo,
    pub editor_plugin_info: EditorPluginInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editor_configuration: Option<EditorConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_proxy: Option<NetworkProxy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_provider: Option<AuthProvider>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Options>,
}

impl Request for SetEditorInfo {
    type Params = SetEditorInfoParams;

    type Result = String;

    // TODO: what is the command to use here??
    const METHOD: &'static str = "setEditorInfo";
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditorInfo {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditorPluginInfo {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditorConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_editor_completions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auto_completions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay_completions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_completions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_languages: Option<Vec<LanguageId>>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LanguageId {
    pub language_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkProxy {
    pub host: String,
    pub port: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_unauthorized: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthProvider {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    // ?
}
