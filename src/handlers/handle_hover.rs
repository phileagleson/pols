use log::info;
use tower_lsp::lsp_types::{
    Hover, HoverContents, HoverParams, MarkupContent, MarkupKind, Position, Range,
};
use tree_sitter::Point;

use crate::{database::account_record_fields::ACCOUNT_RECORD_FIELDS, lsp::CONTEXT};

pub fn handle_hover(params: &HoverParams) -> Option<Hover> {
    info!("received hover request ");
    let line = params.text_document_position_params.position.line as usize;
    let col = params.text_document_position_params.position.character as usize;
    let documents = match CONTEXT.documents.lock() {
        Ok(documents) => documents.clone(),
        Err(_) => return None,
    };

    let document = match documents.get(
        &params
            .text_document_position_params
            .text_document
            .uri
            .to_string(),
    ) {
        Some(document) => document,
        None => return None,
    };
    let trees = match CONTEXT.trees.lock() {
        Ok(trees) => trees.clone(),
        Err(_) => return None,
    };

    let tree = match trees.get(
        &params
            .text_document_position_params
            .text_document
            .uri
            .to_string(),
    ) {
        Some(tree) => tree.clone(),
        None => return None,
    };

    let p = Point {
        row: line,
        column: col,
    };

    let node = tree.root_node().named_descendant_for_point_range(p, p);
    info!("found node kind  {} ", node?.kind());
    match node {
        Some(node) => match node.kind() {
            "field_name" => {
                let field_name = match node.utf8_text(document.text.as_bytes()) {
                    Ok(field_name) => field_name,
                    Err(_) => return None,
                };
                let field_name = field_name.to_lowercase();
                let field_name = field_name.trim();
                info!("field name {} ", field_name);
                // get parent node
                let mut parent_node = match node.parent() {
                    Some(parent_node) => parent_node,
                    None => return None,
                };
                if parent_node.kind() == "field_name" {
                    parent_node = match parent_node.prev_named_sibling() {
                        Some(parent_node) => parent_node,
                        None => return None,
                    };
                };
                info!("parent node kind {} ", parent_node.kind());
                if parent_node.kind() == "record_type" {
                    let record_type = match parent_node.utf8_text(document.text.as_bytes()) {
                        Ok(record_type) => record_type,
                        Err(_) => return None,
                    };
                    let record_type = record_type.to_lowercase().replace(":", "");
                    let record_type = record_type.trim();
                    match record_type {
                        "account" => {
                            let start_pos = Position {
                                line: node.start_position().row as u32,
                                character: node.start_position().column as u32,
                            };
                            let end_pos = Position {
                                line: node.end_position().row as u32,
                                character: node.end_position().column as u32,
                            };

                            let range = Range {
                                start: start_pos,
                                end: end_pos,
                            };

                            match ACCOUNT_RECORD_FIELDS.get(field_name) {
                                Some(field) => {
                                    let hover = Hover {
                                        contents: HoverContents::Markup(MarkupContent {
                                            kind: MarkupKind::Markdown,
                                            value: field.details.to_string(),
                                        }),
                                        range: Some(range),
                                    };
                                    return Some(hover);
                                }
                                None => return None,
                            };
                        }
                        _ => return None,
                    }
                }
                return None;
            }
            _ => return None,
        },
        None => return None,
    }
}
