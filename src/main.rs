// Deny usage of print and eprint as it won't have same result
// in WASI as if doing in standard program, you must really know
// what you are doing to disable that lint (and you don't know)
#![deny(clippy::print_stdout)]
#![deny(clippy::print_stderr)]

use std::{cell::Cell, io::Write, path::PathBuf, process::Stdio, rc::Rc};

use anyhow::Result;
use jsonrpc_lite::{Id, JsonRpc, Params};
use lapce_plugin::{
    psp_types::{
        lsp_types::{
            request::Initialize, DocumentFilter, DocumentSelector, InitializeParams, MessageType,
            Url,
        },
        Request,
    },
    register_plugin, LapcePlugin, VoltEnvironment, PLUGIN_RPC,
};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value};

#[derive(Default)]
struct State {
    id: Rc<Cell<i64>>,
    copilot: Option<std::process::Child>,
}
impl State {
    /// Get copilot's stdin
    fn stdin(&mut self) -> &mut std::process::ChildStdin {
        self.copilot.as_mut().unwrap().stdin.as_mut().unwrap()
    }

    // /// Send a request to copilot
    // pub fn request<P: Serialize, D: DeserializeOwned>(&mut self, method: &str, params: P) -> anyhow::Result<D> {
    //     let id = self.id.get();
    //     self.id.set(id + 1);

    //     let id = Id::Num(id);
    //     let params = Params::from(serde_json::to_value(params).unwrap());
    //     let req = JsonRpc::request_with_params(id, method, params);
    //     let req = serde_json::to_string(&req)?;

    //     let mut stdin = self.stdin();

    //     let _ = stdin.write_all(req.as_bytes());

    //     let mut msg = String::new();
    //     std::io::stdin().read_line(&mut msg)?;

    //     match JsonRpc::parse(&msg) {
    //         Ok(rpc) => {
    //             if let Some(value) = rpc.get_result() {
    //                 let result = serde_json::from_value
    //             }
    //         }
    //     }
    // }
}

register_plugin!(State);

// TODO: Icon in corner for when copilot is doing stuff and whether it is active
// TODO: Copilot generation pane
// TODO: Swap between generations
// TODO: Copilot Chat support
// TODO: the other copilot stuff
fn initialize(state: &mut State, params: InitializeParams) -> Result<()> {
    PLUGIN_RPC.stderr("HELP ME");
    PLUGIN_RPC.window_log_message(MessageType::ERROR, "Initializing copilot".to_string())?;
    PLUGIN_RPC.stderr("HELP ME 2");
    // let document_selector: DocumentSelector = vec![DocumentFilter {
    //     // lsp language id
    //     // language: Some(String::from("language_id")),
    //     language: None,
    //     // TODO: The files it is activated on I think is handled by the plugin itself, we should mimic how the vscode version behaves of not being enabled by default.
    //     // glob pattern
    //     pattern: Some(String::from("**")),
    //     // like file:
    //     scheme: None,
    // }];
    let document_selector: DocumentSelector = vec![DocumentFilter {
        language: Some(String::from("toml")),
        pattern: Some(String::from("**/Cargo.toml")),
        scheme: None,
    }];

    PLUGIN_RPC.stderr("Getting node path");
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

    PLUGIN_RPC.stderr(&format!("Got node path: {node_path:?}"));

    PLUGIN_RPC.window_log_message(MessageType::ERROR, "Checking node version".to_string())?;

    if !check_node_version(node_path.to_string())? {
        PLUGIN_RPC.stderr("NODE VERSION WAS BAD OR SOMETHING?");
        return Ok(());
    }

    PLUGIN_RPC.stderr("Node version was good");

    PLUGIN_RPC.window_log_message(
        MessageType::ERROR,
        "Everything was fine. Starting LSP".to_string(),
    )?;

    //..
    // let path = PathBuf::from("./dist/fun.js");
    // let path = path.canonicalize()?;
    let volt_uri = std::env::var("VOLT_URI")?;
    let volt_uri = volt_uri.strip_prefix("file://").unwrap_or(&volt_uri);
    PLUGIN_RPC.stderr(&format!("VOLT URI: {volt_uri:?}"));
    let file_name = "dist/agent.js";
    let agent_path = std::path::Path::new(&volt_uri).join(file_name);
    let args = vec![agent_path.to_string_lossy().to_string()];
    // let options = None;
    PLUGIN_RPC.stderr(&format!("STARTING LSP: {node_url} {args:?}"));
    let child = std::process::Command::new(node_path)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    PLUGIN_RPC.stderr("Started");

    state.copilot = Some(child);

    // state.request(
    //     Initialize::METHOD,
    //     InitializeParams {
    //         workspace_folders: None,
    //         ..Default::default()
    //     },
    // );

    // let res = PLUGIN_RPC.start_lsp(node_url, args, document_selector, options);
    // match res {
    //     Ok(_) => PLUGIN_RPC.stderr("Started LSP"),
    //     Err(err) => PLUGIN_RPC.stderr(&format!("Failed to start LSP: {err:?}")),
    // }

    // let res = PLUGIN_RPC.execute_process(node_path.to_string(), args)?;

    // let mut server_args = vec![];

    // // Check for user specified LSP server path
    // // ```
    // // [lapce-plugin-name.lsp]
    // // serverPath = "[path or filename]"
    // // serverArgs = ["--arg1", "--arg2"]
    // // ```
    // if let Some(options) = params.initialization_options.as_ref() {
    //     if let Some(lsp) = options.get("lsp") {
    //         if let Some(args) = lsp.get("serverArgs") {
    //             if let Some(args) = args.as_array() {
    //                 if !args.is_empty() {
    //                     server_args = vec![];
    //                 }
    //                 for arg in args {
    //                     if let Some(arg) = arg.as_str() {
    //                         server_args.push(arg.to_string());
    //                     }
    //                 }
    //             }
    //         }

    //         if let Some(server_path) = lsp.get("serverPath") {
    //             if let Some(server_path) = server_path.as_str() {
    //                 if !server_path.is_empty() {
    //                     let server_uri = Url::parse(&format!("urn:{}", server_path))?;
    //                     PLUGIN_RPC.start_lsp(
    //                         server_uri,
    //                         server_args,
    //                         document_selector,
    //                         params.initialization_options,
    //                     );
    //                     return Ok(());
    //                 }
    //             }
    //         }
    //     }
    // }

    // // Architecture check
    // let _ = match VoltEnvironment::architecture().as_deref() {
    //     Ok("x86_64") => "x86_64",
    //     Ok("aarch64") => "aarch64",
    //     _ => return Ok(()),
    // };

    // // OS check
    // let _ = match VoltEnvironment::operating_system().as_deref() {
    //     Ok("macos") => "macos",
    //     Ok("linux") => "linux",
    //     Ok("windows") => "windows",
    //     _ => return Ok(()),
    // };

    // // Download URL
    // // let _ = format!("https://github.com/<name>/<project>/releases/download/<version>/{filename}");

    // // see lapce_plugin::Http for available API to download files

    // let _ = match VoltEnvironment::operating_system().as_deref() {
    //     Ok("windows") => {
    //         format!("{}.exe", "[filename]")
    //     }
    //     _ => "[filename]".to_string(),
    // };

    // // Plugin working directory
    // let volt_uri = VoltEnvironment::uri()?;
    // let server_uri = Url::parse(&volt_uri)?.join("[filename]")?;

    // // if you want to use server from PATH
    // // let server_uri = Url::parse(&format!("urn:{filename}"))?;

    // // Available language IDs
    // // https://github.com/lapce/lapce/blob/HEAD/lapce-proxy/src/buffer.rs#L173
    // PLUGIN_RPC.start_lsp(
    //     server_uri,
    //     server_args,
    //     document_selector,
    //     params.initialization_options,
    // );

    Ok(())
}

impl LapcePlugin for State {
    fn handle_request(&mut self, _id: u64, method: String, params: Value) {
        #[allow(clippy::single_match)]
        match method.as_str() {
            Initialize::METHOD => {
                let params: InitializeParams = serde_json::from_value(params).unwrap();
                if let Err(e) = initialize(self, params) {
                    let _ = PLUGIN_RPC.window_show_message(
                        MessageType::ERROR,
                        format!("plugin returned with error: {e}"),
                    );
                }
            }
            _ => {}
        }
    }
}

fn check_node_version(node: String) -> Result<bool> {
    PLUGIN_RPC.stderr("Checking node version");
    let node_version = PLUGIN_RPC.execute_process(node, vec!["--version".to_string()]);
    PLUGIN_RPC.stderr("Ran process...");
    match node_version {
        Ok(res) => {
            PLUGIN_RPC.stderr("Got result");
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
            return Ok(false);
        }
    }
}
