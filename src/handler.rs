use async_trait::async_trait;
use rust_mcp_sdk::schema::{
    CallToolRequest, CallToolResult, ListToolsRequest, ListToolsResult, RpcError,
    schema_utils::CallToolError,
};
use rust_mcp_sdk::{McpServer, mcp_server::ServerHandler};

use crate::tools::GreetingTools;

pub struct RimeServerHandler;

#[async_trait]
impl ServerHandler for RimeServerHandler {
    // Handle ListToolsRequest, return list of available tools as ListToolsResult
    async fn handle_list_tools_request(
        &self,
        _request: ListToolsRequest,
        _runtime: &dyn McpServer,
    ) -> std::result::Result<ListToolsResult, RpcError> {
        Ok(ListToolsResult {
            meta: None,
            next_cursor: None,
            tools: GreetingTools::tools(),
        })
    }

    async fn handle_call_tool_request(
        &self,
        request: CallToolRequest,
        _runtime: &dyn McpServer,
    ) -> std::result::Result<CallToolResult, CallToolError> {
        let tool_params: GreetingTools =
            GreetingTools::try_from(request.params).map_err(CallToolError::new)?;

        match tool_params {
            GreetingTools::SayHelloTool(say_hello_tool) => say_hello_tool.call_tool(),
            GreetingTools::SayGoodbyeTool(say_goodbye_tool) => say_goodbye_tool.call_tool(),
        }
    }
}
