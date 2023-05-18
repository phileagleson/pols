use log::error;
use tower_lsp::{
    lsp_types::{MessageType, Url},
    Client,
};

use crate::lsp::CONTEXT;
use crate::utils::{analyze, get_files_in_dir, read_document_from_url};

pub async fn handle_initialized(client: &Client) {
    let workspace_folders = match client.workspace_folders().await {
        Ok(folders) => match folders {
            Some(folders) => folders,
            None => {
                client
                    .log_message(
                        MessageType::WARNING,
                        "No workspace folders found".to_string(),
                    )
                    .await;
                return;
            }
        },
        Err(e) => {
            client
                .log_message(
                    MessageType::ERROR,
                    format!("Error getting workspace folders: {}", e),
                )
                .await;
            return;
        }
    };

    let mut files: Vec<String> = Vec::new();
    for folder in workspace_folders {
        client
            .log_message(
                MessageType::LOG,
                format!("Adding workspace folder Folder: {}", folder.name),
            )
            .await;
        let mut files_to_append = match get_files_in_dir(folder.uri.to_string()) {
            Ok(files) => files,
            Err(e) => {
                error!("Error getting files in dir: {}", e);
                return;
            }
        };
        files.append(&mut files_to_append);
    }

    for file in files {
        let url = match Url::parse(&file) {
            Ok(url) => url,
            Err(e) => {
                client
                    .log_message(MessageType::ERROR, format!("Error parsing file url: {}", e))
                    .await;
                return;
            }
        };

        let text_document = read_document_from_url(url.clone()).await;
        let mut documents = match CONTEXT.documents.lock() {
            Ok(documents) => documents,
            Err(e) => {
                error!("Error getting documents lock: {}", e);
                return;
            }
        };
        documents.insert(url.to_string(), text_document.clone());

        match analyze(&text_document) {
            Ok(tree) => match tree {
                Some(tree) => {
                    CONTEXT
                        .trees
                        .lock()
                        .unwrap()
                        .insert(text_document.uri.to_string(), tree.clone());
                    tree
                }
                None => {
                    return;
                }
            },

            Err(_) => {
                return;
            }
        };
    }

    client
        .log_message(MessageType::INFO, "PowerOn LSP initialized".to_string())
        .await;
}
