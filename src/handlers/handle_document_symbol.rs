use log::error;
use tower_lsp::lsp_types::{
    DocumentSymbolParams, DocumentSymbolResponse, Location, Position, Range, SymbolInformation,
    SymbolKind, TextDocumentItem,
};
use tree_sitter::{Query, QueryCursor};

use crate::{lsp::CONTEXT, utils::get_inc_files};

pub fn handle_document_symbol(params: &DocumentSymbolParams) -> Option<DocumentSymbolResponse> {
    let documents = match CONTEXT.documents.lock() {
        Ok(documents) => documents.clone(),
        Err(e) => {
            error!("Error getting documents: {}", e.to_string());
            return None;
        }
    };

    let document = match documents.get(&params.text_document.uri.to_string()) {
        Some(document) => document.clone(),
        None => {
            error!("No document found for {}", params.text_document.uri);
            return None;
        }
    };

    let files_to_search = get_files_to_search(&document);
    let mut symbols: Vec<SymbolInformation> = Vec::new();
    for file in files_to_search {
        let file_doc = match documents.get(&file) {
            Some(document) => document.clone(),
            None => {
                error!("No document found for {}", file);
                return None;
            }
        };
        let file_symbols = match get_document_symbols(&file_doc) {
            Some(symbols) => symbols,
            None => {
                error!("Error getting document symbols");
                return None;
            }
        };
        symbols.extend(file_symbols);
    }
    Some(DocumentSymbolResponse::Flat(symbols))
}

#[allow(deprecated)]
pub fn get_document_symbols(text_document: &TextDocumentItem) -> Option<Vec<SymbolInformation>> {
    let var_query_string = "(variable_declaration (identifier) @ident)";
    let proc_query_string = "(procedure_definition (identifier) @proc)";

    let trees = match CONTEXT.trees.lock() {
        Ok(trees) => trees.clone(),
        Err(e) => {
            error!("Error getting trees: {}", e.to_string());
            return None;
        }
    };

    let tree = match trees.get(&text_document.uri.to_string()) {
        Some(tree) => tree.clone(),
        None => {
            error!("No tree found for {}", text_document.uri);
            return None;
        }
    };

    let source = text_document.text.as_str();
    let lang = tree.language();
    let mut var_cursor = QueryCursor::new();
    let mut proc_cursor = QueryCursor::new();

    let var_query = match Query::new(lang, var_query_string) {
        Ok(query) => query,
        Err(e) => {
            error!("Error creating query: {}", e.to_string());
            return None;
        }
    };
    let proc_query = match Query::new(lang, proc_query_string) {
        Ok(query) => query,
        Err(e) => {
            error!("Error creating query: {}", e.to_string());
            return None;
        }
    };
    let var_matches = var_cursor.matches(&var_query, tree.root_node(), source.as_bytes());
    let proc_matches = proc_cursor.matches(&proc_query, tree.root_node(), source.as_bytes());

    let mut doc_symbols: Vec<SymbolInformation> = Vec::new();
    for m in var_matches {
        let node_parent = match m.captures[0].node.parent() {
            Some(node) => node,
            None => {
                error!("Error getting parent node");
                continue;
            }
        };

        let var_dec_range: Range = Range {
            start: Position {
                line: node_parent.start_position().row as u32,
                character: node_parent.start_position().column as u32,
            },
            end: Position {
                line: node_parent.end_position().row as u32,
                character: node_parent.end_position().column as u32,
            },
        };
        let name = match m.captures[0].node.utf8_text(source.as_bytes()) {
            Ok(name) => name,
            Err(e) => {
                error!("Error getting name: {}", e.to_string());
                continue;
            }
        };
        let symbol_location: Location = Location {
            uri: text_document.uri.clone(),
            range: var_dec_range,
        };

        let symbol: SymbolInformation = SymbolInformation {
            name: name.to_string(),
            kind: SymbolKind::VARIABLE,
            tags: None,
            deprecated: None,
            location: symbol_location,
            container_name: None,
        };
        doc_symbols.push(symbol.clone());
    }

    for m in proc_matches {
        let node_parent = match m.captures[0].node.parent() {
            Some(node) => node,
            None => {
                error!("Error getting parent node");
                continue;
            }
        };

        let proc_def_range: Range = Range {
            start: Position {
                line: node_parent.start_position().row as u32,
                character: node_parent.start_position().column as u32,
            },
            end: Position {
                line: node_parent.end_position().row as u32,
                character: node_parent.end_position().column as u32,
            },
        };
        let name = match m.captures[0].node.utf8_text(source.as_bytes()) {
            Ok(name) => name,
            Err(e) => {
                error!("Error getting name: {}", e.to_string());
                continue;
            }
        };
        let symbol_location: Location = Location {
            uri: text_document.uri.clone(),
            range: proc_def_range,
        };

        let symbol: SymbolInformation = SymbolInformation {
            name: name.to_string(),
            kind: SymbolKind::FUNCTION,
            tags: None,
            deprecated: None,
            location: symbol_location,
            container_name: None,
        };
        doc_symbols.push(symbol.clone());
    }

    Some(doc_symbols)
}

fn get_files_to_search(document: &TextDocumentItem) -> Vec<String> {
    let mut files_to_search: Vec<String> = Vec::new();
    let mut searched_files: Vec<String> = Vec::new();
    let inc_files = get_inc_files(&document);
    searched_files.push(document.uri.to_string());
    // search 3 levels deep
    for _ in 1..3 {
        inc_files.iter().for_each(|f| {
            if !searched_files.contains(f) {
                let doc = match CONTEXT.documents.lock() {
                    Ok(documents) => {
                        let doc = match documents.get(f) {
                            Some(doc) => doc.clone(),
                            None => {
                                error!("error getting document for uri: {}", f);
                                return;
                            }
                        };
                        doc
                    }
                    Err(e) => {
                        error!("error getting documents lock: {}", e);
                        return;
                    }
                };
                let mut inc_files = get_inc_files(&doc);
                if !files_to_search.contains(f) {
                    files_to_search.push(f.to_string());
                }
                if inc_files.len() > 0 {
                    searched_files.push(f.to_string());
                    files_to_search.append(&mut inc_files);
                }
            }
        });
    }
    files_to_search
}
