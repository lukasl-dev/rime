use std::sync::Arc;

use async_trait::async_trait;
use rust_mcp_sdk::schema::{
    CallToolRequestParams, CallToolResult, ListToolsResult, PaginatedRequestParams, RpcError,
    schema_utils::CallToolError,
};
use rust_mcp_sdk::{McpServer, mcp_server::ServerHandler};

use crate::tools::RimeTools;

pub struct RimeServerHandler;

#[async_trait]
impl ServerHandler for RimeServerHandler {
    async fn handle_list_tools_request(
        &self,
        _request: Option<PaginatedRequestParams>,
        _runtime: Arc<dyn McpServer>,
    ) -> std::result::Result<ListToolsResult, RpcError> {
        Ok(ListToolsResult {
            meta: None,
            next_cursor: None,
            tools: RimeTools::tools(),
        })
    }

    async fn handle_call_tool_request(
        &self,
        params: CallToolRequestParams,
        _runtime: Arc<dyn McpServer>,
    ) -> std::result::Result<CallToolResult, CallToolError> {
        let tool_params: RimeTools =
            RimeTools::try_from(params).map_err(CallToolError::new)?;

        match tool_params {
            RimeTools::NixEvaluateTool(tool) => tool.call_tool(),
            RimeTools::NixLogTool(tool) => tool.call_tool(),
            RimeTools::NixPackagesSearchTool(tool) => tool.call_tool(),
            RimeTools::NixPackagesWhyDepends(tool) => tool.call_tool(),
            RimeTools::NixFlakesShowTool(tool) => tool.call_tool(),
            RimeTools::NixFlakesMetadataTool(tool) => tool.call_tool(),
            RimeTools::NixConfigShowTool(tool) => tool.call_tool(),
            RimeTools::NixOSChannelsTool(tool) => tool.call_tool(),
            RimeTools::NixManualListTool(tool) => tool.call_tool(),
            RimeTools::NixManualReadTool(tool) => tool.call_tool(),
            RimeTools::NixOSWikiSearchTool(tool) => tool.call_tool(),
            RimeTools::NixOSWikiReadTool(tool) => tool.call_tool(),
            RimeTools::NixConfigCheckTool(tool) => tool.call_tool(),
            RimeTools::ManixSearchTool(tool) => tool.call_tool(),
            RimeTools::NixHubPackageVersionsTool(tool) => tool.call_tool(),
            RimeTools::HomeManagerOptionsSearch(tool) => tool.call_tool(),
            RimeTools::NvfOptionsSearchTool(tool) => tool.call_tool(),
            RimeTools::NvfManualListTool(tool) => tool.call_tool(),
            RimeTools::NvfManualReadTool(tool) => tool.call_tool(),
            RimeTools::NixpkgsOptionsSearchTool(tool) => tool.call_tool(),
        }
    }
}
