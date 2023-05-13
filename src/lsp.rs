use std::collections::HashMap;

use std::process::exit;
use std::sync::Mutex;

use lazy_static::lazy_static;

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};
use tree_sitter::{Parser, Tree};

use crate::handlers::handle_completion::handle_comlpetion;
use crate::handlers::handle_did_change_text_document::handle_did_change_text_document;
use crate::handlers::handle_document_symbol::handle_document_symbol;
use crate::handlers::handle_initialized::handle_initialized;
use crate::handlers::{handle_definition, handle_hover::handle_hover};
use crate::{handlers::handle_initialize::handle_initialize, parser::get_parser};

pub struct Backend {
    pub client: Client,
}

pub struct Context {
    pub documents: Mutex<HashMap<String, TextDocumentItem>>,
    pub parser: Mutex<Parser>,
    pub trees: Mutex<HashMap<String, Tree>>,
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

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        handle_did_change_text_document(&params);
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let result = handle_document_symbol(&params);
        Ok(result)
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let res = handle_hover(&params);
        Ok(res)
    }

    async fn shutdown(&self) -> Result<()> {
        exit(0)
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let completions = handle_comlpetion(&params);
        Ok(completions)
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let result = handle_definition::handle_definition(&params);
        Ok(result)
    }
}
