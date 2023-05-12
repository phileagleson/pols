use tower_lsp::lsp_types::*;

pub fn handle_initialize() -> InitializeResult {
    let file_operation_filter = FileOperationFilter {
        scheme: None,
        pattern: FileOperationPattern {
            glob: String::from("**/*.{git}"),
            matches: None,
            options: Some(FileOperationPatternOptions {
                ignore_case: Some(true),
            }),
        },
    };

    let folder_operation_filter = FileOperationFilter {
        scheme: None,
        pattern: FileOperationPattern {
            glob: String::from("**/*"),
            matches: None,
            options: None,
        },
    };

    InitializeResult {
        capabilities: ServerCapabilities {
            position_encoding: None,
            text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
            selection_range_provider: None,
            hover_provider: None,
            completion_provider: Some(CompletionOptions {
                resolve_provider: Some(true),
                trigger_characters: Some(Vec::from([
                    ":".to_string(),
                    "=".to_string(),
                    "@".to_string(),
                ])),
                all_commit_characters: None,
                work_done_progress_options: Default::default(),
                completion_item: Some(CompletionOptionsCompletionItem {
                    label_details_support: None,
                }),
            }),
            signature_help_provider: None,
            definition_provider: Some(OneOf::Left(true)),
            type_definition_provider: None,
            implementation_provider: None,
            references_provider: None,
            document_highlight_provider: None,
            document_symbol_provider: None,
            workspace_symbol_provider: None,
            code_action_provider: None,
            code_lens_provider: None,
            document_formatting_provider: None,
            document_range_formatting_provider: None,
            document_on_type_formatting_provider: None,
            rename_provider: None,
            document_link_provider: None,
            color_provider: None,
            folding_range_provider: None,
            declaration_provider: None,
            execute_command_provider: None,
            workspace: Some(WorkspaceServerCapabilities {
                workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                    supported: Some(true),
                    change_notifications: Some(OneOf::Left(true)),
                }),
                file_operations: Some(WorkspaceFileOperationsServerCapabilities {
                    did_create: Some(FileOperationRegistrationOptions {
                        filters: vec![
                            file_operation_filter.clone(),
                            folder_operation_filter.clone(),
                        ],
                    }),
                    did_rename: Some(FileOperationRegistrationOptions {
                        filters: vec![
                            file_operation_filter.clone(),
                            folder_operation_filter.clone(),
                        ],
                    }),
                    did_delete: Some(FileOperationRegistrationOptions {
                        filters: vec![
                            file_operation_filter.clone(),
                            folder_operation_filter.clone(),
                        ],
                    }),
                    will_delete: Some(FileOperationRegistrationOptions {
                        filters: vec![
                            file_operation_filter.clone(),
                            folder_operation_filter.clone(),
                        ],
                    }),
                    will_create: None,
                    will_rename: None,
                }),
            }),
            call_hierarchy_provider: None,
            semantic_tokens_provider: None,
            moniker_provider: None,
            inline_value_provider: None,
            inlay_hint_provider: None,
            linked_editing_range_provider: None,
            experimental: None,
        },
        server_info: None,
    }
}
