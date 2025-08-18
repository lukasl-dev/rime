use async_trait::async_trait;
use rust_mcp_sdk::schema::{
    CallToolRequest, CallToolResult, ListToolsRequest, ListToolsResult, RpcError,
    schema_utils::CallToolError,
};
use rust_mcp_sdk::{McpServer, mcp_server::ServerHandler};

use crate::tools::RimeTools;

pub struct RimeServerHandler;

#[async_trait]
impl ServerHandler for RimeServerHandler {
    async fn handle_list_tools_request(
        &self,
        _request: ListToolsRequest,
        _runtime: &dyn McpServer,
    ) -> std::result::Result<ListToolsResult, RpcError> {
        Ok(ListToolsResult {
            meta: None,
            next_cursor: None,
            tools: RimeTools::tools(),
        })
    }

    async fn handle_call_tool_request(
        &self,
        request: CallToolRequest,
        _runtime: &dyn McpServer,
    ) -> std::result::Result<CallToolResult, CallToolError> {
        let tool_params: RimeTools =
            RimeTools::try_from(request.params).map_err(CallToolError::new)?;

        match tool_params {
            RimeTools::PackagesSearchTool(tool) => tool.call_tool(),
            RimeTools::PackagesWhyDepends(tool) => tool.call_tool(),
            RimeTools::FlakesShowTool(tool) => tool.call_tool(),
            RimeTools::WikiSearchTool(tool) => tool.call_tool(),
            RimeTools::WikiGetPageTool(tool) => tool.call_tool(),
            RimeTools::ManixSearchTool(tool) => tool.call_tool(),
        }
    }
}
