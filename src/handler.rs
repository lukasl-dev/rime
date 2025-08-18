use async_trait::async_trait;
use rust_mcp_sdk::schema::{
    CallToolRequest, CallToolResult, ListToolsRequest, ListToolsResult, RpcError,
    schema_utils::CallToolError,
};
use rust_mcp_sdk::{McpServer, mcp_server::ServerHandler};

use crate::tools::RimTools;

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
            tools: RimTools::tools(),
        })
    }

    async fn handle_call_tool_request(
        &self,
        request: CallToolRequest,
        _runtime: &dyn McpServer,
    ) -> std::result::Result<CallToolResult, CallToolError> {
        let tool_params: RimTools =
            RimTools::try_from(request.params).map_err(CallToolError::new)?;

        match tool_params {
            RimTools::PackagesSearchTool(tool) => tool.call_tool(),
            RimTools::PackagesWhyDepends(tool) => tool.call_tool(),
            RimTools::FlakeShowTool(tool) => tool.call_tool(),
        }
    }
}
