use log::error;
use tower_lsp::lsp_types::{
    GotoDefinitionParams, GotoDefinitionResponse, Location, TextDocumentItem,
};
use tree_sitter::{Point, Query, QueryCursor};

use crate::{
    lsp::CONTEXT,
    utils::{is_def_file, is_poweron_driver},
};

#[derive(Clone, Debug)]
pub enum QueryType {
    Identifier,
    ProcedureCall,
}

pub fn handle_definition(params: &GotoDefinitionParams) -> Option<GotoDefinitionResponse> {
    let mut result: Vec<Location> = Vec::new();
    let line = params.text_document_position_params.position.line as usize;
    let col = params.text_document_position_params.position.character as usize;
    let p = Point::new(line, col);
    let trees = match CONTEXT.trees.lock() {
        Ok(trees) => trees.clone(),
        Err(e) => {
            error!("error getting trees lock: {}", e);
            return None;
        }
    };
    let tree = match trees.get(
        &params
            .text_document_position_params
            .text_document
            .uri
            .to_string(),
    ) {
        Some(tree) => tree.clone(),
        None => {
            error!(
                "error getting tree for uri: {} on line {}",
                params.text_document_position_params.text_document.uri, 42
            );
            return None;
        }
    };

    let node_to_find = tree.root_node().descendant_for_point_range(p, p);
    if node_to_find.is_none() {
        return None;
    }
    let node_to_find = match node_to_find {
        Some(node) => node,
        None => {
            error!("error getting node to find ");
            return None;
        }
    };
    if node_to_find.kind() != "identifier" && node_to_find.kind() != "procedure_call" {
        return None;
    }

    let documents = match CONTEXT.documents.lock() {
        Ok(documents) => documents.clone(),
        Err(e) => {
            error!("error getting documents lock: {}", e);
            return None;
        }
    };

    let document = match documents.get(
        &params
            .text_document_position_params
            .text_document
            .uri
            .to_string(),
    ) {
        Some(document) => document.clone(),
        None => {
            error!(
                "error getting document for uri: {}",
                params.text_document_position_params.text_document.uri
            );
            return None;
        }
    };

    if is_def_file(&document) {
        return None;
    }

    let source = &document.text.as_bytes();
    let mut query_type: QueryType = QueryType::Identifier;
    let mut declaration_to_find = match node_to_find.utf8_text(source) {
        Ok(text) => text,
        Err(e) => {
            error!("error getting utf8 text: {}", e);
            return None;
        }
    };

    if node_to_find.kind() == "procedure_call" {
        query_type = QueryType::ProcedureCall;
        let child = node_to_find.child_by_field_name("identifier");
        if child.is_none() {
            return None;
        }
        let child_to_find = match child {
            Some(child) => child,
            None => {
                error!("error getting child to find");
                return None;
            }
        };
        declaration_to_find = match child_to_find.utf8_text(source) {
            Ok(text) => text,
            Err(e) => {
                error!("error getting utf8 text: {}", e);
                return None;
            }
        };
    }

    if node_to_find.parent().is_some() {
        let parent = match node_to_find.parent() {
            Some(parent) => parent,
            None => {
                error!("error getting parent");
                return None;
            }
        };
        if parent.kind() == "procedure_call" {
            query_type = QueryType::ProcedureCall;
        }
    }

    result.append(&mut search_current_file(
        &document,
        query_type.clone(),
        declaration_to_find,
    ));

    if result.len() > 0 {
        return Some(GotoDefinitionResponse::Array(result));
    }

    let files_to_search = get_files_to_seach(&document);

    for file in files_to_search {
        let documents = match CONTEXT.documents.lock() {
            Ok(documents) => documents.clone(),
            Err(e) => {
                error!("error getting documents lock: {}", e);
                return None;
            }
        };
        let document = match documents.get(&file) {
            Some(document) => document.clone(),
            None => {
                error!("error getting document for uri: {}", file);
                return None;
            }
        };
        result.append(&mut search_current_file(
            &document,
            query_type.clone(),
            declaration_to_find,
        ))
    }

    Some(GotoDefinitionResponse::Array(result))
}

fn search_current_file(
    document: &TextDocumentItem,
    query_type: QueryType,
    declaration_to_find: &str,
) -> Vec<Location> {
    let mut result: Vec<Location> = Vec::new();
    let source = document.text.as_str();
    let trees = match CONTEXT.trees.lock() {
        Ok(trees) => trees.clone(),
        Err(e) => {
            error!("error getting trees lock: {}", e);
            return result;
        }
    };

    let tree = match trees.get(&document.uri.to_string()) {
        Some(tree) => tree.clone(),
        None => {
            error!(
                "error getting tree for uri: {} on line {}",
                document.uri, 196
            );
            return result;
        }
    };

    let lang = tree.language();
    let mut cursor = QueryCursor::new();
    let query_string = match query_type {
        QueryType::Identifier => "(variable_declaration (identifier) @ident)",
        QueryType::ProcedureCall => "(procedure_definition (identifier) @ident)",
    };

    let query = match Query::new(lang, query_string) {
        Ok(query) => query,
        Err(e) => {
            error!("error creating query: {}", e);
            return result;
        }
    };

    let matches = cursor.matches(&query, tree.root_node(), source.as_bytes());
    matches.for_each(|m| {
        let ident = match m.captures[0].node.utf8_text(source.as_bytes()) {
            Ok(ident) => ident,
            Err(e) => {
                error!("error getting utf8 text: {}", e);
                return;
            }
        };
        if ident == declaration_to_find {
            // we found a match in this file return it.
            let start = m.captures[0].node.start_position();
            let end = m.captures[0].node.end_position();
            let location = Location {
                uri: document.uri.clone(),
                range: tower_lsp::lsp_types::Range {
                    start: tower_lsp::lsp_types::Position {
                        line: start.row as u32,
                        character: start.column as u32,
                    },
                    end: tower_lsp::lsp_types::Position {
                        line: end.row as u32,
                        character: end.column as u32,
                    },
                },
            };

            result.push(location);
        }
    });
    result
}

fn get_files_to_seach(document: &TextDocumentItem) -> Vec<String> {
    let mut files_to_search: Vec<String> = Vec::new();
    // Attempting to be smart if the poweron is a 'driver' file. In this case,
    // we only find the definition if it is in the same file or in an include file.
    // this should alievieate finding multiple definitions in the same workspace for
    // common vars like true/false
    if is_poweron_driver(&document) {
        // find all inc files
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
    } else {
        // not driver file search all files in the workspace
        let documents = match CONTEXT.documents.lock() {
            Ok(documents) => documents.clone(),
            Err(e) => {
                error!("error getting documents lock: {}", e);
                return Vec::new();
            }
        };

        documents.iter().for_each(|(uri, _)| {
            if !files_to_search.contains(uri) {
                files_to_search.push(uri.to_string());
            }
        });
    }

    files_to_search
}

pub fn get_inc_files(document: &TextDocumentItem) -> Vec<String> {
    let query_string = "(include_statement (string_literal) @inc)";
    // find all inc def files
    let source = document.text.as_str();
    let trees = match CONTEXT.trees.lock() {
        Ok(trees) => trees.clone(),
        Err(e) => {
            error!("error getting trees lock: {}", e);
            return Vec::new();
        }
    };

    let tree = match trees.get(&document.uri.to_string()) {
        Some(tree) => tree.clone(),
        None => {
            error!(
                "error getting tree for uri: {} on line {}",
                document.uri, 312
            );
            return Vec::new();
        }
    };
    let lang = tree.language();
    let mut cursor = QueryCursor::new();
    let query = match Query::new(lang, query_string) {
        Ok(query) => query,
        Err(e) => {
            error!("error creating query: {}", e);
            return Vec::new();
        }
    };

    let matches = cursor.matches(&query, tree.root_node(), source.as_bytes());
    let mut inc_files: Vec<String> = Vec::new();
    matches.for_each(|m| {
        let inc = match m.captures[0].node.utf8_text(source.as_bytes()) {
            Ok(inc) => inc,
            Err(e) => {
                error!("error getting utf8 text: {}", e);
                return;
            }
        };
        let inc = inc.replace("\"", "");
        // see if the document exist in the workspace
        let documents = match CONTEXT.documents.lock() {
            Ok(documents) => documents.clone(),
            Err(e) => {
                error!("error getting documents lock: {}", e);
                return;
            }
        };
        documents.iter().for_each(|(uri, doc)| {
            if doc.uri.to_string().contains(&inc) {
                inc_files.push(uri.to_string());
            }
        });
    });
    inc_files
}
