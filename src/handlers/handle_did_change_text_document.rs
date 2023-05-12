use log::info;
use tower_lsp::lsp_types::{DidChangeTextDocumentParams, TextDocumentItem};

use crate::{lsp::CONTEXT, utils::analyze};

pub fn handle_did_change_text_document(params: &DidChangeTextDocumentParams) {
    info!("received didChangeTextDocument notification");
    // find the document and the tree
    let mut documents = match CONTEXT.documents.lock() {
        Ok(documents) => documents,
        Err(e) => {
            info!("failed to lock documents: {}", e);
            return;
        }
    };

    let document = &mut TextDocumentItem {
        uri: params.text_document.uri.clone(),
        language_id: "poweron".to_string(),
        version: params.text_document.version,
        text: params.content_changes[0].text.clone(),
    };
    documents.insert(params.text_document.uri.to_string(), document.clone());
    let mut trees = match CONTEXT.trees.lock() {
        Ok(trees) => trees,
        Err(e) => {
            info!("failed to lock trees: {}", e);
            return;
        }
    };
    let tree = analyze(document);
    match tree {
        Ok(tree) => match tree {
            Some(tree) => {
                trees.insert(params.text_document.uri.to_string(), tree);
            }
            None => {
                info!("failed to parse document");
                return;
            }
        },
        Err(e) => {
            info!("failed to parse document: {}", e);
            return;
        }
    }
}
