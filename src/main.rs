mod handler;
mod tools;

use handler::RimeServerHandler;
use rust_mcp_sdk::mcp_server::{HyperServerOptions, hyper_server};
use rust_mcp_sdk::schema::{
    Implementation, InitializeResult, LATEST_PROTOCOL_VERSION, ServerCapabilities,
    ServerCapabilitiesTools,
};

use rust_mcp_sdk::{
    McpServer, StdioTransport, TransportOptions,
    error::SdkResult,
    mcp_server::{ServerRuntime, server_runtime},
};

#[tokio::main]
async fn main() -> SdkResult<()> {
    let server_details = InitializeResult {
        server_info: Implementation {
            name: "rime".to_string(),
            version: "0.1.0".to_string(),
            title: Some("rime".to_string()),
        },
        capabilities: ServerCapabilities {
            tools: Some(ServerCapabilitiesTools { list_changed: None }),
            ..Default::default()
        },
        meta: None,
        instructions: None,
        protocol_version: LATEST_PROTOCOL_VERSION.to_string(),
    };

    let transport = StdioTransport::new(TransportOptions::default())?;
    let handler = RimeServerHandler {};

    // let server = hyper_server::create_server(
    //     server_details,
    //     handler,
    //     HyperServerOptions {
    //         host: "127.0.0.1".to_string(),
    //         sse_support: false,
    //         ..Default::default()
    //     },
    // );

    let server: ServerRuntime = server_runtime::create_server(server_details, transport, handler);
    server.start().await
}
