use rust_mcp_sdk::schema::{CallToolResult, TextContent, schema_utils::CallToolError};
use rust_mcp_sdk::{
    macros::{JsonSchema, mcp_tool},
    tool_box,
};

#[mcp_tool(
    name = "say_hello",
    description = "Accepts a person's name and says a personalized \"Hello\" to that person"
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct SayHelloTool {
    /// The name of the person to greet with a "Hello".
    name: String,
}

impl SayHelloTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let hello_message = format!("Hello, {}!", self.name);
        Ok(CallToolResult::text_content(vec![TextContent::from(
            hello_message,
        )]))
    }
}

#[mcp_tool(
    name = "say_goodbye",
    description = "Accepts a person's name and says a personalized \"Goodbye\" to that person."
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct SayGoodbyeTool {
    /// The name of the person to say goodbye to.
    name: String,
}
impl SayGoodbyeTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let hello_message = format!("Goodbye, {}!", self.name);
        Ok(CallToolResult::text_content(vec![TextContent::from(
            hello_message,
        )]))
    }
}

tool_box!(GreetingTools, [SayHelloTool, SayGoodbyeTool]);
