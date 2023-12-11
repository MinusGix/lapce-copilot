// Deny usage of print and eprint as it won't have same result
// in WASI as if doing in standard program, you must really know
// what you are doing to disable that lint (and you don't know)
#![deny(clippy::print_stdout)]
#![deny(clippy::print_stderr)]

use std::collections::HashMap;

use anyhow::Result;
use copilot::{
    CheckAuthStatus, CheckAuthStatusParams, CheckAuthStatusResult, EditorConfiguration, EditorInfo,
    EditorPluginInfo, GetCompletions, GetCompletionsCycling, GetCompletionsResult, SetEditorInfo,
    SetEditorInfoParams, SignInConfirm, SignInConfirmParams, SignInConfirmResult, SignInInitiate,
    SignInInitiateParams, SignInInitiateResult, SignInStatus, Status,
};

use lapce_plugin::{
    lsp::LspRef,
    psp_types::{
        lsp_types::{
            notification::{DidChangeTextDocument, DidOpenTextDocument},
            request::{Initialize, InlineCompletionRequest},
            DidChangeTextDocumentParams, DidOpenTextDocumentParams, DocumentFilter,
            DocumentSelector, InitializeParams, InitializeResult, InlineCompletionItem,
            InlineCompletionParams, InlineCompletionResponse, InlineCompletionTriggerKind,
            InsertTextFormat, MessageType, OneOf, ServerCapabilities, ServerInfo, TextDocumentItem,
            TextDocumentSyncCapability, TextDocumentSyncKind, TextDocumentSyncOptions,
            TextDocumentSyncSaveOptions, Url, VersionedTextDocumentIdentifier,
        },
        Notification, Request,
    },
    register_plugin, LapcePlugin, VoltEnvironment, PLUGIN_RPC,
};

use serde_json::Value;

pub mod copilot;

pub const PLUGIN_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
struct State {
    pub lsp: Option<LspRef>,
    /// Track the latest versions
    pub versions: HashMap<Url, i32>,
}
impl State {
    fn handle_inline_completion(&mut self, id: u64, params: InlineCompletionParams) -> Result<()> {
        PLUGIN_RPC.stderr("Handling Inline Completion");
        let Some(lsp) = self.lsp else {
            return Ok(());
        };

        let InlineCompletionParams {
            text_document_position,
            context,
            ..
        } = params;

        let method = match context.trigger_kind {
            InlineCompletionTriggerKind::Automatic => GetCompletions::METHOD,
            InlineCompletionTriggerKind::Invoked => GetCompletionsCycling::METHOD,
            _ => {
                PLUGIN_RPC.stderr(&format!(
                    "Unsupported trigger kind: {:?}",
                    context.trigger_kind
                ));
                return Ok(());
            }
        };

        let version = self
            .versions
            .get(&text_document_position.text_document.uri)
            .copied()
            .unwrap_or_else(|| {
                PLUGIN_RPC.stderr(&format!(
                    "No version for uri: {:?}",
                    text_document_position.text_document.uri
                ));
                0
            });

        let params = copilot::GetCompletionsParams {
            doc: copilot::GetCompletionsDoc {
                position: text_document_position.position,
                uri: text_document_position.text_document.uri.clone(),
                version,
                insert_spaces: None,
                tab_size: None,
                source: None,
                language_id: None,
                relative_path: None,
                if_inserted: None,
            },
            options: None,
        };

        let params = serde_json::to_value(params).unwrap();

        PLUGIN_RPC.stderr(&format!(
            "URI: {:?}; VERSION: {:?}; sending to lsp",
            &text_document_position.text_document.uri, version
        ));
        let GetCompletionsResult { completions } = lsp.send_request_blocking(method, params)?;

        PLUGIN_RPC.stderr(&format!("Got completions: {completions:?}"));

        let completions: Vec<_> = completions
            .into_iter()
            .map(|c| {
                InlineCompletionItem {
                    // TODO: is this correct?
                    insert_text: c.display_text,
                    insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
                    filter_text: None,
                    range: None, // TODO
                    command: None,
                }
            })
            .collect();
        let params = InlineCompletionResponse::Array(completions);
        let params = serde_json::to_value(params).unwrap();

        PLUGIN_RPC.stderr("Sending success");
        PLUGIN_RPC.host_success(id, params)?;

        Ok(())
    }

    fn handle_did_change_text_document(&mut self, params: DidChangeTextDocumentParams) {
        let DidChangeTextDocumentParams { text_document, .. } = params;

        let VersionedTextDocumentIdentifier { uri, version } = text_document;

        self.versions.insert(uri, version);
    }

    fn handle_did_open_text_document(&mut self, params: DidOpenTextDocumentParams) {
        let DidOpenTextDocumentParams { text_document, .. } = params;

        let TextDocumentItem { uri, version, .. } = text_document;

        self.versions.insert(uri, version);
    }
}

register_plugin!(State);

// TODO: Icon in corner for when copilot is doing stuff and whether it is active
// TODO: Copilot generation panel
// TODO: Swap between generations, though that's a Lapce thing
// TODO: Copilot Chat support
// TODO: the other copilot stuff
fn initialize(state: &mut State, params: InitializeParams) -> Result<()> {
    PLUGIN_RPC.window_log_message(MessageType::ERROR, "Initializing copilot".to_string())?;
    let document_selector: DocumentSelector = vec![DocumentFilter {
        language: None,
        pattern: Some(String::from("**/*")),
        scheme: None,
    }];

    // By default we just try using some global node
    let mut node_url = Url::parse("urn:node")?;
    let mut node_path = "node";
    if let Some(options) = params.initialization_options.as_ref() {
        if let Some(node) = options.get("node") {
            if let Some(path) = node.get("path") {
                if let Some(path) = path.as_str() {
                    node_url = Url::parse(&format!("urn:{}", path))?;
                    node_path = path;
                }
            }
        }
    }

    if !check_node_version(node_path.to_string())? {
        PLUGIN_RPC.stderr("NODE VERSION WAS BAD OR SOMETHING?");
        return Ok(());
    }

    PLUGIN_RPC.window_log_message(
        MessageType::ERROR,
        "Everything was fine. Starting LSP".to_string(),
    )?;

    let volt_uri = std::env::var("VOLT_URI")?;
    let volt_uri = volt_uri.strip_prefix("file://").unwrap_or(&volt_uri);
    let file_name = "dist/agent.js";
    let agent_path = std::path::Path::new(&volt_uri).join(file_name);
    let args = vec![agent_path.to_string_lossy().to_string()];

    let lsp = PLUGIN_RPC.start_lsp(
        node_url,
        args,
        document_selector,
        params.initialization_options,
    )?;

    state.lsp = Some(lsp);

    let resp: String = lsp.send_request_blocking(
        SetEditorInfo::METHOD,
        SetEditorInfoParams {
            editor_info: EditorInfo {
                name: "Lapce".to_string(),
                version: "0.3.1".to_string(),
            },
            editor_plugin_info: EditorPluginInfo {
                name: "lapce-copilot".to_string(),
                version: PLUGIN_VERSION.to_string(),
            },
            editor_configuration: Some(EditorConfiguration {
                show_editor_completions: Some(true),
                enable_auto_completions: Some(true),
                disabled_languages: Some(vec![]),
                ..Default::default()
            }),
            auth_provider: None,
            network_proxy: None,
            options: None,
        },
    )?;

    if resp != "OK" {
        PLUGIN_RPC.stderr(&format!(
            "RESPONSE TO Copilot's setEditorInfo WAS NOT OK: {resp:?}"
        ));
    }

    let status: CheckAuthStatusResult = lsp.send_request_blocking(
        CheckAuthStatus::METHOD,
        CheckAuthStatusParams { options: None },
    )?;

    if status.status == Status::Ok {
        PLUGIN_RPC
            .window_log_message(MessageType::INFO, "Copilot already signed in".to_string())?;
        return Ok(());

        // // Sign out for testing
        // let resp: SignOutResult = lsp.send_request_blocking(SignOut::METHOD, SignOutParams {})?;
        // PLUGIN_RPC.stderr(&format!("SIGN OUT RESULT: {resp:?}"));

        // let status: CheckAuthStatusResult = lsp.send_request_blocking(
        //     CheckAuthStatus::METHOD,
        //     CheckAuthStatusParams { options: None },
        // )?;

        // PLUGIN_RPC.stderr(&format!("AUTH STATUS NEW: {status:?}"));
    }

    // Log in
    let resp: SignInInitiateResult =
        lsp.send_request_blocking(SignInInitiate::METHOD, SignInInitiateParams {})?;

    match resp.status {
        SignInStatus::AlreadySignedIn => {
            PLUGIN_RPC.window_log_message(
                MessageType::WARNING,
                "Already signed-in despite checking that..".to_string(),
            )?;
            return Ok(());
        }
        SignInStatus::PromptUserDeviceFlow => {
            let Some(verification_uri) = &resp.verification_uri else {
                PLUGIN_RPC
                    .window_log_message(MessageType::ERROR, "No verification uri".to_string())?;
                anyhow::bail!("No verification uri: {resp:?}");
            };
            let Some(user_code) = &resp.user_code else {
                PLUGIN_RPC.window_log_message(
                    MessageType::ERROR,
                    "No user code for sign-in".to_string(),
                )?;
                anyhow::bail!("No user code: {resp:?}");
            };
            let message = format!("Input this code in the opened browser: {}", user_code);
            PLUGIN_RPC.window_show_message(MessageType::INFO, message)?;

            open(verification_uri)?;
        }
    }

    let _resp: SignInConfirmResult =
        lsp.send_request_blocking(SignInConfirm::METHOD, SignInConfirmParams {})?;

    Ok(())
}

fn open(url: &str) -> anyhow::Result<()> {
    let os = VoltEnvironment::operating_system()?;
    match os.as_str() {
        "linux" | "freebsd" | "netbsd" | "openbsd" | "solaris" | "android" => {
            let _ = PLUGIN_RPC.execute_process("xdg-open".to_string(), vec![url.to_string()])?;
        }
        "macos" => {
            let _ = PLUGIN_RPC.execute_process("open".to_string(), vec![url.to_string()])?;
        }
        "windows" => {
            let _ = PLUGIN_RPC.execute_process(
                "cmd".to_string(),
                vec!["/C".to_string(), "start".to_string(), url.to_string()],
            )?;
        }
        _ => {
            let err = format!("Unsupported operating system {os:?} when trying to open {url}, please open it manually.");
            PLUGIN_RPC.window_show_message(MessageType::ERROR, err.clone())?;
            return Ok(());
        }
    }

    Ok(())
}

// The Copilot agent.js uses a custom `getCompletions`/`getCompletionsCycle` request for inline
// completions, but we don't want to force Lapce to support the non-standard request.
//
// Thankfully, the 3.18 version of the LSP spec comes with inline completion support - though it is
// not yet finalized.
// What this plugin does then, is start copilot and tell Lapce that the plugin supports inline
// completions. The plugin then translates back and forth between the inline completion request and
// the custom request that copilot expects.
//
// Once Copilot properly supports inlineCompletion requests, this extra code can be removed.

/// Reply to the initialize request properly
fn reply_initialize(id: u64, _params: &InitializeParams) -> Result<()> {
    let message = InitializeResult {
        capabilities: ServerCapabilities {
            inline_completion_provider: Some(OneOf::Left(true)),
            // We don't care about the file contents, but we need to be alerted so that we can
            // track the version number for copilot.
            text_document_sync: Some(TextDocumentSyncCapability::Options(
                TextDocumentSyncOptions {
                    change: Some(TextDocumentSyncKind::INCREMENTAL),
                    open_close: Some(true),
                    save: Some(TextDocumentSyncSaveOptions::Supported(true)),
                    ..Default::default()
                },
            )),
            ..Default::default()
        },
        server_info: Some(ServerInfo {
            name: "lapce-copilot".to_string(),
            version: Some(PLUGIN_VERSION.to_string()),
        }),
        ..Default::default()
    };

    PLUGIN_RPC.host_success(id, message)?;

    Ok(())
}

impl LapcePlugin for State {
    fn handle_request(&mut self, id: u64, method: String, params: Value) {
        match method.as_str() {
            Initialize::METHOD => {
                let params: InitializeParams = serde_json::from_value(params).unwrap();

                if let Err(e) = reply_initialize(id, &params) {
                    let _ = PLUGIN_RPC.window_show_message(
                        MessageType::ERROR,
                        format!("plugin reply_initialize returned with error: {e}"),
                    );
                }

                if let Err(e) = initialize(self, params) {
                    let _ = PLUGIN_RPC.window_show_message(
                        MessageType::ERROR,
                        format!("plugin returned with error: {e}"),
                    );
                }
            }
            InlineCompletionRequest::METHOD => {
                let params: Result<InlineCompletionParams, _> = serde_json::from_value(params);
                let params = match params {
                    Ok(params) => params,
                    Err(err) => {
                        PLUGIN_RPC
                            .stderr(&format!("Failed to parse inline completion params: {err}"));
                        return;
                    }
                };

                if let Err(e) = self.handle_inline_completion(id, params) {
                    PLUGIN_RPC.stderr(&format!("copilot inline completion error: {e}"));
                }
            }
            _ => {}
        }
    }

    fn handle_notification(&mut self, method: String, params: Value) {
        match method.as_str() {
            DidChangeTextDocument::METHOD => {
                let params: DidChangeTextDocumentParams = serde_json::from_value(params).unwrap();

                self.handle_did_change_text_document(params);
            }
            DidOpenTextDocument::METHOD => {
                let params: DidOpenTextDocumentParams = serde_json::from_value(params).unwrap();

                self.handle_did_open_text_document(params);
            }
            _ => {}
        }
    }
}

fn check_node_version(node: String) -> Result<bool> {
    let node_version = PLUGIN_RPC.execute_process(node, vec!["--version".to_string()]);
    match node_version {
        Ok(res) => {
            if !res.success {
                PLUGIN_RPC.window_show_message(
                    MessageType::ERROR,
                    "Node.js did not successfully exit.".to_string(),
                )?;
                PLUGIN_RPC.stderr("Node.js did not successfully exit.");
                return Ok(false);
            }

            let Some(stdout) = res.stdout else {
                let err = "Failed to get stdout when getting Nodejs version".to_string();
                PLUGIN_RPC.window_show_message(MessageType::ERROR, err.clone())?;
                PLUGIN_RPC.stderr(&err);
                PLUGIN_RPC.window_log_message(MessageType::ERROR, err)?;
                return Ok(false);
            };

            // Node's version is typically of the form v16.16.0
            let stdout = std::str::from_utf8(&stdout)?;
            let stdout = stdout.trim();

            // Currently we just require >16.0.0
            // We just do a kinda hacky strip
            let version = stdout.strip_prefix('v').unwrap_or(stdout);
            let (version, _) = version.split_once('.').unwrap_or(("", ""));
            let Ok(version) = version.parse::<u32>() else {
                let err = format!("Failed to parse Nodejs version: {:?}", stdout);
                PLUGIN_RPC.window_show_message(MessageType::ERROR, err.clone())?;
                PLUGIN_RPC.stderr(&err);
                PLUGIN_RPC.window_log_message(MessageType::ERROR, err)?;
                return Ok(false);
            };

            if version < 16 {
                let err = format!(
                    "Node.js version is too old, we require a minimum of v16: {:?}",
                    stdout
                );
                PLUGIN_RPC.window_show_message(MessageType::ERROR, err.clone())?;
                PLUGIN_RPC.stderr(&err);
                PLUGIN_RPC.window_log_message(MessageType::ERROR, err)?;
                return Ok(false);
            }

            Ok(true)
        }
        Err(err) => {
            let err = format!("Node.js failed to start: {}", err);
            PLUGIN_RPC.window_show_message(MessageType::ERROR, err.clone())?;
            PLUGIN_RPC.stderr(&err);
            PLUGIN_RPC.window_log_message(MessageType::ERROR, err)?;
            Ok(false)
        }
    }
}
