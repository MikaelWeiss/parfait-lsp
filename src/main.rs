use lsp_server::{Connection, Message, Request, Response};
use lsp_types::{
    InitializeParams, InitializeResult, TextDocumentItem, TextDocumentPositionParams,
    CompletionParams, CompletionList, CompletionItem, CompletionItemKind,
    HoverParams, Hover, MarkupContent, MarkupKind,
    Position, Range, Url,
};
use std::sync::Arc;
use std::path::PathBuf;

fn main() {
    // Set up a connection to the language server
    let (connection, io_handles) = Connection::stdio();
    let server_capabilities = serde_json::to_value(&initialize_capabilities()).unwrap();

    // Main loop that processes incoming messages
    for message in connection.receiver() {
        if let Err(err) = handle_message(&message, &connection) {
            error!("Error handling message: {}", err);
            break;
        }
    }

    // Clean up IO handles
    if let Err(err) = io_handles.join() {
        error!("Error joining IO handles: {:?}", err);
    }
}

fn handle_message(msg: &Message, conn: &Connection) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match msg {
        Message::Request(req) => {
            if let Err(err) = handle_request(req, conn) {
                error!("Error handling request: {:?}", err);
            }
        },
        Message::Response(res) => {
            if let Err(err) = handle_response(res, conn) {
                error!("Error handling response: {:?}", err);
            }
        },
        Message::Notification(not) => {
            handle_notification(not, conn)?;
        },
    }
    Ok(())
}

fn handle_request(req: &Request, conn: &Connection) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match req.method.as_str() {
        "initialize" => {
            let params: InitializeParams = serde_json::from_value(req.params.clone())?;
            let result = handle_initialize(params)?;
            conn.reply(req, result)?;
        },
        "textDocument/completion" => {
            let params: CompletionParams = serde_json::from_value(req.params.clone())?;
            let result = handle_completion(params)?;
            conn.reply(req, result)?;
        },
        "textDocument/hover" => {
            let params: HoverParams = serde_json::from_value(req.params.clone())?;
            let result = handle_hover(params)?;
            conn.reply(req, result)?;
        },
        _ => {
            conn.reply(req, Response::error(
                req.id.clone(),
                lsp_server::ErrorCode::MethodNotFound as i32,
                format!("Method not found: {}", req.method),
            ))?;
        },
    }
    Ok(())
}

fn handle_initialize(params: InitializeParams) -> Result<InitializeResult, Box<dyn std::error::Error + Send + Sync>> {
    // Initialize capabilities and return result
    Ok(InitializeResult {
        capabilities: server_capabilities.clone(),
        server_info: None,
    })
}

fn handle_completion(params: CompletionParams) -> Result<CompletionList, Box<dyn std::error::Error + Send + Sync>> {
    // Implement completion logic based on params
    let items = vec![
        CompletionItem {
            label: "queryable".to_string(),
            kind: Some(CompletionItemKind::Keyword),
            detail: Some("Attribute: Makes field queryable".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "default".to_string(),
            kind: Some(CompletionItemKind::Keyword),
            detail: Some("Attribute: Sets default value for field".to_string()),
            ..Default::default()
        },
        // Add more completion items as needed
    ];

    Ok(CompletionList {
        is_incomplete: false,
        items,
    })
}

fn handle_hover(params: HoverParams) -> Result<Option<Hover>, Box<dyn std::error::Error + Send + Sync>> {
    // Implement hover logic based on params
    let content = MarkupContent {
        kind: MarkupKind::Markdown,
        value: format!("Hover information for `{}`", params.text_document_position_params.text_document.uri),
    };

    Ok(Some(Hover {
        contents: content,
        range: None,
    }))
}

fn initialize_capabilities() -> lsp_types::ServerCapabilities {
    // Define the capabilities of the language server
    lsp_types::ServerCapabilities {
        text_document_sync: Some(lsp_types::TextDocumentSyncCapability::Kind(lsp_types::TextDocumentSyncKind::Full)),
        hover_provider: Some(true),
        completion_provider: Some(lsp_types::CompletionOptions {
            resolve_provider: None,
            trigger_characters: Some(vec![".".to_string(), ":".to_string()]),
        }),
        // Add more capabilities as needed
        ..Default::default()
    }
}

