mod handler;
mod tools;

use clap::{Parser, Subcommand};
use handler::RimeServerHandler;
use rust_mcp_sdk::error::SdkResult;
use rust_mcp_sdk::mcp_server::{HyperServerOptions, ServerRuntime, hyper_server, server_runtime};
use rust_mcp_sdk::schema::{
    Implementation, InitializeResult, LATEST_PROTOCOL_VERSION, ServerCapabilities,
    ServerCapabilitiesTools,
};
use rust_mcp_sdk::{McpServer, StdioTransport, TransportOptions};

#[derive(Parser, Debug)]
#[command(
    name = "rime",
    version,
    about = "Rime MCP server",
    propagate_version = true,
    subcommand_required = true,
    arg_required_else_help = true
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Run the MCP server over stdio (default if no subcommand is provided)
    Stdio,
    /// Run the MCP server over HTTP
    Http(HttpArgs),
}

#[derive(Parser, Debug, Clone)]
struct HttpArgs {
    /// Host to bind (default: 127.0.0.1)
    #[arg(long, default_value = "127.0.0.1")]
    host: String,

    /// Port to bind (default: 8080)
    #[arg(long, default_value_t = 8080)]
    port: u16,

    /// Enable SSE support for HTTP transport
    #[arg(long = "sse", default_value_t = true)]
    sse: bool,

    /// Enable JSON responses for HTTP transport (non-streaming)
    #[arg(long = "json", default_value_t = false)]
    json: bool,
}

fn server_details() -> InitializeResult {
    InitializeResult {
        server_info: Implementation {
            name: "rime".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            title: Some("rime".to_string()),
        },
        capabilities: ServerCapabilities {
            tools: Some(ServerCapabilitiesTools { list_changed: None }),
            ..Default::default()
        },
        meta: None,
        instructions: None,
        protocol_version: LATEST_PROTOCOL_VERSION.to_string(),
    }
}

#[tokio::main]
async fn main() -> SdkResult<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Stdio => run_stdio().await,
        Command::Http(args) => run_http(args).await,
    }
}

async fn run_stdio() -> SdkResult<()> {
    let transport = StdioTransport::new(TransportOptions::default())?;
    let handler = RimeServerHandler {};
    let server: ServerRuntime = server_runtime::create_server(server_details(), transport, handler);
    server.start().await
}

async fn run_http(args: HttpArgs) -> SdkResult<()> {
    let handler = RimeServerHandler {};
    let server = hyper_server::create_server(
        server_details(),
        handler,
        HyperServerOptions {
            host: args.host,
            port: args.port,
            sse_support: args.sse,
            enable_json_response: Some(args.json),
            ..Default::default()
        },
    );
    server.start().await
}
