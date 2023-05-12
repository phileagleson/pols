use log4rs;
use pols::lsp::Backend;
use tower_lsp::{LspService, Server};

#[tokio::main]
async fn main() {
    log4rs::init_file(
        "/home/phil/projects/pols/logging_config.yaml",
        Default::default(),
    )
    .unwrap();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend { client });

    Server::new(stdin, stdout, socket).serve(service).await;
}
