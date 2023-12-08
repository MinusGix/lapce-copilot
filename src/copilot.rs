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

#[derive(Debug)]
pub enum CheckAuthStatus {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckAuthStatusParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<CheckAuthStatusOptions>,
}

impl Request for CheckAuthStatus {
    type Params = CheckAuthStatusParams;

    type Result = CheckAuthStatusResult;

    const METHOD: &'static str = "checkStatus";
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckAuthStatusOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_checks_only: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckAuthStatusResult {
    /// "OK" | "NotSignedIn" | ?
    pub status: String,
    /// Github user
    pub user: Option<String>,
}

/// Start signing in  
/// The user will have to open the verification uri and then enter the user code
#[derive(Debug)]
pub enum SignInInitiate {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignInInitiateParams {}

impl Request for SignInInitiate {
    type Params = SignInInitiateParams;

    type Result = SignInInitiateResult;

    const METHOD: &'static str = "signInInitiate";
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignInInitiateResult {
    // TODO: could we make this an enum...?
    /// "PromptUserDeviceFlow" | "AlreadySignedIn"
    pub status: String,
    /// Only for "PromptUserDeviceFlow"
    /// Short string of numbers
    pub user_code: Option<String>,
    /// Only for "PromptUserDeviceFlow"
    pub verification_uri: Option<String>,
    /// Only for "PromptUserDeviceFlow"
    pub expires_in: Option<f32>,
    /// Only for "PromptUserDeviceFlow"
    pub interval: Option<f32>,

    /// Only for "AlreadySignedIn"
    pub user: Option<String>,
}

/// Sign in confirm waits until the user has finished interacting to respond
#[derive(Debug)]
pub enum SignInConfirm {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignInConfirmParams {}

impl Request for SignInConfirm {
    type Params = SignInConfirmParams;

    type Result = SignInConfirmResult;

    const METHOD: &'static str = "signInConfirm";
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignInConfirmResult {
    /// "OK" | ?
    pub status: String,
    pub user: Option<String>,
}

#[derive(Debug)]
pub enum SignOut {}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignOutParams {}

impl Request for SignOut {
    type Params = SignOutParams;

    type Result = SignOutResult;

    const METHOD: &'static str = "signOut";
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignOutResult {
    /// "NotSignedin" | ?
    pub status: String,
}
