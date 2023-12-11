use lapce_plugin::psp_types::{
    lsp_types::{Position, Range, Url},
    Notification, Request,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "OK")]
    Ok,
    MaybeOk,
    NotSignedIn,
    NotAuthorized,
    FailedToGetToken,
    TokenInvalid,
}
impl Status {
    pub fn is_ok(self) -> bool {
        self == Status::Ok || self == Status::MaybeOk
    }
}

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
    pub options: Option<Value>,
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
    pub status: Status,
    /// Github user
    pub user: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SignInStatus {
    AlreadySignedIn,
    PromptUserDeviceFlow,
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
    pub status: SignInStatus,
    /// Only for [`Status::PromptUserDeviceFlow`]
    /// Short string of numbers
    pub user_code: Option<String>,
    /// Only for [`Status::PromptUserDeviceFlow`]
    pub verification_uri: Option<String>,
    /// Only for [`Status::PromptUserDeviceFlow`]
    pub expires_in: Option<f32>,
    /// Only for [`Status::PromptUserDeviceFlow`]
    pub interval: Option<f32>,

    /// Only for [`Status::AlreadySignedIn`]
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

#[derive(Debug)]
pub enum GetCompletions {}

impl Request for GetCompletions {
    type Params = GetCompletionsParams;

    type Result = GetCompletionsResult;

    const METHOD: &'static str = "getCompletions";
}

#[derive(Debug)]
pub enum GetCompletionsCycling {}

impl Request for GetCompletionsCycling {
    type Params = GetCompletionsParams;

    type Result = GetCompletionsResult;

    const METHOD: &'static str = "getCompletionsCycling";
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCompletionsParams {
    pub doc: GetCompletionsDoc,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCompletionsDoc {
    pub position: Position,
    pub uri: Url,
    pub version: i32,
    /// Whether to insert spaces maybe?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_spaces: Option<bool>,
    /// The size of tabs in the document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab_size: Option<u16>,
    /// ??
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_inserted: Option<IfInserted>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IfInserted {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<Position>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCompletionsResult {
    pub completions: Vec<Completion>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Completion {
    pub uuid: String,
    /// Full line of text
    pub text: String,
    pub range: Range,
    /// Text after the current cursor position
    pub display_text: String,
    pub position: Position,
    pub doc_version: u64,
}

/// I think this is supposed to be sent when the completion is shown?
#[derive(Debug)]
pub enum NotifyShown {}

impl Notification for NotifyShown {
    type Params = NotifyShownParams;

    const METHOD: &'static str = "notifyShown";
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotifyShownParams {
    pub uuid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Value>,
}

/// This is supposed to be sent when a completion is accepted
#[derive(Debug)]
pub enum NotifyAccepted {}

impl Notification for NotifyAccepted {
    type Params = NotifyAcceptedParams;

    const METHOD: &'static str = "notifyAccepted";
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotifyAcceptedParams {
    pub uuid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Value>,
}

/// This is supposed to be sent when a completion is rejected
#[derive(Debug)]
pub enum NotifyRejected {}

impl Notification for NotifyRejected {
    type Params = NotifyRejectedParams;

    const METHOD: &'static str = "notifyRejected";
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotifyRejectedParams {
    pub uuid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Value>,
}

#[derive(Debug)]
pub enum Cancel {}

impl Request for Cancel {
    type Params = CancelParams;

    type Result = CancelResult;

    const METHOD: &'static str = "cancel";
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CancelParams {
    /// Request id
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CancelResult {
    // ??
}
