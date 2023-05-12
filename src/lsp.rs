use std::collections::HashMap;

use std::process::exit;
use std::sync::Mutex;

use lazy_static::lazy_static;
use log::info;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};
use tree_sitter::{Parser, Tree};

use crate::completions::poweron_functions::POWERON_FUNCTION_COMPLETIONS;
use crate::handlers::handle_definition;
use crate::handlers::handle_initialized::handle_initialized;
use crate::{handlers::handle_initialize::handle_initialize, parser::get_parser};

pub struct Backend {
    pub client: Client,
}

pub struct Context {
    pub documents: Mutex<HashMap<String, TextDocumentItem>>,
    pub parser: Mutex<Parser>,
    pub trees: Mutex<HashMap<String, Tree>>,
    pub symbols: Mutex<Vec<SymbolInformation>>,
}

lazy_static! {
    pub static ref CONTEXT: Context = Context::new();
}

impl Context {
    pub fn new() -> Self {
        Self {
            documents: Mutex::new(HashMap::new()),
            parser: Mutex::new(get_parser()),
            trees: Mutex::new(HashMap::new()),
            symbols: Mutex::new(Vec::new()),
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        let result = handle_initialize();
        Ok(result)
    }

    async fn initialized(&self, _: InitializedParams) {
        handle_initialized(&self.client).await;
        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        exit(0)
    }

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        let default_completions: Vec<CompletionItem> =
            POWERON_FUNCTION_COMPLETIONS.values().cloned().collect();
        info!("completion request received");
        info!("default_completions: {:?}", default_completions);
        Ok(Some(CompletionResponse::Array(default_completions)))
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        info!("goto_definition request received");
        let result = handle_definition::handle_definition(&params);
        info!("goto_definition result: {:?}", result);
        Ok(result)
    }
}
