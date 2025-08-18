use std::io::Error;

use rust_mcp_sdk::schema::{CallToolResult, TextContent, schema_utils::CallToolError};
use rust_mcp_sdk::{
    macros::{JsonSchema, mcp_tool},
    tool_box,
};

#[mcp_tool(
    name = "packages.search",
    description = "Searches for packages in a given installable, such as `nixpkgs`."
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct PackagesSearchTool {
    /// The name of the installable to search in.
    ///
    /// Examples: "nixpkgs", "github:owner/repo", "gitlab:owner/repo", etc.
    installable: String,

    /// The regex pattern to search for packages.
    ///
    /// Examples: "git", "^cargo", etc.
    regex: String,
}

impl PackagesSearchTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        // Run: nix search --json <installable> <regex>
        let output = std::process::Command::new("nix")
            .args([
                "search",
                "--json",
                self.installable.as_str(),
                self.regex.as_str(),
            ])
            .output()
            .map_err(CallToolError::new)?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let err = Error::other(format!(
                r#"nix search failed (status: {}): {}"#,
                output.status, stderr
            ));
            return Err(CallToolError::new(err));
        }

        let stdout = String::from_utf8(output.stdout).map_err(CallToolError::new)?;
        let json_val: serde_json::Value =
            serde_json::from_str(&stdout).map_err(CallToolError::new)?;

        let pretty = serde_json::to_string_pretty(&json_val).map_err(CallToolError::new)?;

        Ok(CallToolResult::text_content(vec![TextContent::from(
            pretty,
        )]))
    }
}

#[mcp_tool(
    name = "packages.why_depends",
    description = "Show why a package has another package in its closure."
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct PackagesWhyDepends {
    /// The name of the package to check dependencies for.
    ///
    /// Examples: "glibc", "nixpkgs#git", etc.
    package: String,

    /// The name of the dependency to check for.
    ///
    /// Examples: "glibc", "nixpkgs#git", etc.
    dependency: String,
}

impl PackagesWhyDepends {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        // Run: nix why-depends --all <package> <dependency>
        let output = std::process::Command::new("nix")
            .args([
                "why-depends",
                "--all",
                self.package.as_str(),
                self.dependency.as_str(),
            ])
            .output()
            .map_err(CallToolError::new)?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let err = Error::other(format!(
                r#"nix why-depends failed (status: {}): {}"#,
                output.status, stderr
            ));
            return Err(CallToolError::new(err));
        }

        let stdout = String::from_utf8(output.stdout).map_err(CallToolError::new)?;
        Ok(CallToolResult::text_content(vec![TextContent::from(
            stdout,
        )]))
    }
}

#[mcp_tool(
    name = "flake.show",
    description = "Show the outputs provided by a given flake."
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct FlakeShowTool {
    /// The flake to show outputs for.
    ///
    /// Examples: "github:neuro-soup/evochi", "/path/to/nixos/flake/dir", etc.
    flake: String,
}

impl FlakeShowTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        // Run: nix flake show --json <flake>
        let output = std::process::Command::new("nix")
            .args(["flake", "show", "--json", self.flake.as_str()])
            .output()
            .map_err(CallToolError::new)?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let err = Error::other(format!(
                r#"nix flake show (status: {}): {}"#,
                output.status, stderr
            ));
            return Err(CallToolError::new(err));
        }

        let stdout = String::from_utf8(output.stdout).map_err(CallToolError::new)?;
        let json_val: serde_json::Value =
            serde_json::from_str(&stdout).map_err(CallToolError::new)?;

        let pretty = serde_json::to_string_pretty(&json_val).map_err(CallToolError::new)?;

        Ok(CallToolResult::text_content(vec![TextContent::from(
            pretty,
        )]))
    }
}

tool_box!(
    RimTools,
    [PackagesSearchTool, PackagesWhyDepends, FlakeShowTool]
);
