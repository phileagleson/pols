use log::info;
use tower_lsp::lsp_types::{
    CompletionItem, CompletionItemKind, CompletionItemLabelDetails, CompletionParams,
    CompletionResponse, InsertTextFormat,
};

use crate::{
    completions::poweron_functions::POWERON_FUNCTION_COMPLETIONS,
    database::{account_record_fields::ACCOUNT_RECORD_FIELDS, types::DatabaseField},
    utils::word_at_point,
};

pub fn handle_comlpetion(params: &CompletionParams) -> Option<CompletionResponse> {
    match &params.context {
        Some(context) => {
            match &context.trigger_character {
                Some(trigger_character) => {
                    if trigger_character == ":" {
                        info!("Trigger character is :");
                        return handle_database_field(&params);
                    }
                    return None;
                }
                None => return Some(CompletionResponse::Array(get_default_completions())),
            };
        }
        None => return Some(CompletionResponse::Array(get_default_completions())),
    };
}

fn get_default_completions() -> Vec<CompletionItem> {
    let default_completions: Vec<CompletionItem> =
        POWERON_FUNCTION_COMPLETIONS.values().cloned().collect();
    default_completions
}

fn handle_database_field(params: &CompletionParams) -> Option<CompletionResponse> {
    let line = &params.text_document_position.position.line;
    let col = &params.text_document_position.position.character;
    let uri = &params.text_document_position.text_document.uri;
    let database_record = match word_at_point(uri, line, col) {
        Some(database_record) => database_record.to_lowercase().replace(":", ""),
        None => return None,
    };

    match database_record.as_str() {
        "account" => {
            let account_fields: Vec<DatabaseField> =
                ACCOUNT_RECORD_FIELDS.values().cloned().collect();
            let account_fields: Vec<CompletionItem> = account_fields
                .iter()
                .map(|field| {
                    let completion_item = CompletionItem {
                        label: field.mnemonic.clone().to_uppercase(),
                        label_details: Some(CompletionItemLabelDetails {
                            detail: None,
                            description: Some(field.description.to_string()),
                        }),
                        kind: Some(CompletionItemKind::FIELD),
                        detail: Some(field.details.to_string()),
                        deprecated: Some(false),
                        insert_text: Some(field.mnemonic.clone().to_string().to_uppercase()),
                        insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
                        ..CompletionItem::default()
                    };
                    completion_item
                })
                .collect();
            return Some(CompletionResponse::Array(account_fields));
        }
        _ => return None,
    };
}
