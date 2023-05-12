use std::{error::Error, fs, path::Path};

use log::{error, info};
use tower_lsp::lsp_types::{TextDocumentItem, Url};
use tree_sitter::{Point, Query, QueryCursor, Tree};

use crate::lsp::CONTEXT;

type MyResult<T> = Result<T, Box<dyn Error>>;

pub async fn get_files_in_dir(uri: String) -> MyResult<Vec<String>> {
    let mut files: Vec<String> = Vec::new();
    let path = uri.replace("file://", "");
    let path = Path::new(&path);
    let files_in_path = match fs::read_dir(path) {
        Ok(files_in_path) => files_in_path,
        Err(e) => {
            error!("Error reading directory: {}", e.to_string());
            return Ok(files);
        }
    };
    for entry in files_in_path {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                error!("Error reading entry: {}", e.to_string());
                continue;
            }
        };
        let file_path = entry.path();
        if file_path.is_file() {
            let file_name = match file_path.file_name() {
                Some(file_name) => file_name,
                None => {
                    error!("Error getting file name");
                    continue;
                }
            };
            let file_name = match file_name.to_str() {
                Some(file_name) => file_name,
                None => {
                    error!("Error getting file name");
                    continue;
                }
            };
            if !file_name.starts_with(".") {
                let file_name =
                    "file://".to_string() + &file_path.to_str().unwrap().replace("\\", "/");
                files.push(file_name.to_string());
                info!("Adding file: {}", file_name);
            }
        }
    }

    Ok(files)
}

pub async fn read_document_from_url(url: Url) -> TextDocumentItem {
    let file_path = match url.to_file_path() {
        Ok(url) => url,
        Err(_) => {
            return TextDocumentItem {
                uri: url,
                language_id: "poweron".to_string(),
                version: 0,
                text: "".to_string(),
            };
        }
    };

    let content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            error!("Error reading file: {}", e.to_string());
            "".to_string()
        }
    };

    let document = TextDocumentItem {
        uri: url,
        language_id: "poweron".to_string(),
        version: 0,
        text: content,
    };

    document
}

pub fn analyze(text_document: &TextDocumentItem) -> MyResult<Option<Tree>> {
    let mut parser = match CONTEXT.parser.lock() {
        Ok(parser) => parser,
        Err(e) => {
            error!("Error getting parser: {}", e.to_string());
            return Ok(None);
        }
    };
    let tree = parser.parse(text_document.text.as_str(), None);

    return Ok(tree);
}

pub fn is_def_file(text_document: &TextDocumentItem) -> bool {
    let print_query_string = "(print_division) @print";
    let declaration_query_string = "(variable_declaration) @proc";
    let mut has_print_division = false;
    let mut has_variable_declaration = false;

    let trees = match CONTEXT.trees.lock() {
        Ok(trees) => trees.clone(),
        Err(e) => {
            error!("Error getting trees: {}", e.to_string());
            return false;
        }
    };

    let tree = match trees.get(&text_document.uri.to_string()) {
        Some(tree) => tree.clone(),
        None => {
            error!("No tree found for {}", text_document.uri);
            return false;
        }
    };

    let source = text_document.text.as_str();
    let lang = tree.language();
    let mut print_cursor = QueryCursor::new();
    let mut declaration_cursor = QueryCursor::new();
    let print_query = match Query::new(lang, print_query_string) {
        Ok(query) => query,
        Err(e) => {
            error!("Error creating query: {}", e.to_string());
            return false;
        }
    };
    let declaration_query = match Query::new(lang, declaration_query_string) {
        Ok(query) => query,
        Err(e) => {
            error!("Error creating query: {}", e.to_string());
            return false;
        }
    };

    let print_matches = print_cursor.matches(&print_query, tree.root_node(), source.as_bytes());
    let declaration_matches =
        declaration_cursor.matches(&declaration_query, tree.root_node(), source.as_bytes());

    if print_matches.count() > 0 {
        has_print_division = true;
    }

    if declaration_matches.count() > 0 {
        has_variable_declaration = true;
    }

    if !has_print_division && has_variable_declaration {
        return true;
    }

    false
}

pub fn is_pro_file(text_document: &TextDocumentItem) -> bool {
    let print_query_string = "(print_division) @print";
    let procedure_query_string = "(procedure_definition) @proc";
    let mut has_print_division = false;
    let mut has_procedure_definition = false;

    let trees = match CONTEXT.trees.lock() {
        Ok(trees) => trees.clone(),
        Err(e) => {
            error!("Error getting trees: {}", e.to_string());
            return false;
        }
    };

    let tree = match trees.get(&text_document.uri.to_string()) {
        Some(tree) => tree.clone(),
        None => {
            error!("No tree found for {}", text_document.uri);
            return false;
        }
    };

    let source = text_document.text.as_str();
    let lang = tree.language();
    let mut print_cursor = QueryCursor::new();
    let mut procedure_cursor = QueryCursor::new();
    let print_query = match Query::new(lang, print_query_string) {
        Ok(query) => query,
        Err(e) => {
            error!("Error creating query: {}", e.to_string());
            return false;
        }
    };
    let procedure_query = match Query::new(lang, procedure_query_string) {
        Ok(query) => query,
        Err(e) => {
            error!("Error creating query: {}", e.to_string());
            return false;
        }
    };
    let print_matches = print_cursor.matches(&print_query, tree.root_node(), source.as_bytes());
    let procedure_matches =
        procedure_cursor.matches(&procedure_query, tree.root_node(), source.as_bytes());

    if print_matches.count() > 0 {
        has_print_division = true;
    }

    if procedure_matches.count() > 0 {
        has_procedure_definition = true;
    }

    if !has_print_division && has_procedure_definition {
        return true;
    }
    false
}

pub fn is_poweron_driver(text_document: &TextDocumentItem) -> bool {
    let query_string = "(print_division) @print";

    let mut has_print_division = false;
    let trees = match CONTEXT.trees.lock() {
        Ok(trees) => trees.clone(),
        Err(e) => {
            error!("Error getting trees: {}", e.to_string());
            return false;
        }
    };
    let tree = match trees.get(&text_document.uri.to_string()) {
        Some(tree) => tree.clone(),
        None => {
            error!("No tree found for {}", text_document.uri);
            return false;
        }
    };

    let source = text_document.text.as_str();
    let lang = tree.language();
    let mut cursor = QueryCursor::new();
    let query = match Query::new(lang, query_string) {
        Ok(query) => query,
        Err(e) => {
            error!("Error creating query: {}", e.to_string());
            return false;
        }
    };
    let matches = cursor.matches(&query, tree.root_node(), source.as_bytes());
    if matches.count() > 0 {
        has_print_division = true;
    }

    has_print_division
}

pub fn node_at_point(line: usize, col: usize, text_document: &TextDocumentItem) -> Option<String> {
    let p: Point = Point::new(line, col);
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
    let node = tree.root_node().named_descendant_for_point_range(p, p);
    let node_type = match node {
        Some(node) => return Some(node.kind().to_string()),
        None => None,
    };
    node_type
}

pub fn get_basename_from_uri(uri: &str) -> String {
    let uri = uri.trim_start_matches("file://");
    let path = Path::new(uri);
    let file_name = match path.file_name() {
        Some(file_name) => file_name.to_str().unwrap(),
        None => "",
    };
    file_name.to_string()
}
