use std::io::Error;
use std::process::Command;

use rust_mcp_sdk::schema::{CallToolResult, TextContent, schema_utils::CallToolError};
use rust_mcp_sdk::{
    macros::{JsonSchema, mcp_tool},
    tool_box,
};

#[mcp_tool(name = "evaluate", description = "Evaluate a Nix expression.")]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct EvaluateTool {
    /// The Nix expression to evaluate.
    ///
    /// Examples: "nixpkgs#lib.version", etc.
    expression: String,
}

impl EvaluateTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        // Run: nix eval --json <expression>
        let output = Command::new("nix")
            .args([
                "--extra-experimental-features",
                "nix-command flakes",
                "eval",
                "--json",
                self.expression.as_str(),
            ])
            .output()
            .map_err(CallToolError::new)?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let err = Error::other(format!(
                r#"nix eval (status: {}): {}"#,
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

#[mcp_tool(name = "log", description = "Evaluate a Nix expression.")]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct LogTool {
    /// The name of the installable to get the build log for.
    ///
    /// Examples: "nixpkgs", "github:owner/repo", "gitlab:owner/repo", etc.
    installable: String,
}

impl LogTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        // Run: nix search --json <installable> <regex>
        let output = Command::new("nix")
            .args([
                "--extra-experimental-features",
                "nix-command flakes",
                "log",
                self.installable.as_str(),
            ])
            .output()
            .map_err(CallToolError::new)?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let err = Error::other(format!(
                r#"nix log (status: {}): {}"#,
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
    name = "packages_search",
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
        let output = Command::new("nix")
            .args([
                "--extra-experimental-features",
                "nix-command flakes",
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
    name = "packages_why_depends",
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
        let output = Command::new("nix")
            .args([
                "--extra-experimental-features",
                "nix-command flakes",
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
    name = "flakes_show",
    description = "Show the outputs provided by a given flake."
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct FlakesShowTool {
    /// The flake to show outputs for.
    ///
    /// Examples: "github:neuro-soup/evochi", "/path/to/nixos/flake/dir", etc.
    flake: String,
}

impl FlakesShowTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        // Run: nix flake show --json <flake>
        let output = Command::new("nix")
            .args([
                "--extra-experimental-features",
                "nix-command flakes",
                "flake",
                "show",
                "--json",
                self.flake.as_str(),
            ])
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

#[mcp_tool(name = "flakes_metadata", description = "Show flake metadata.")]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct FlakesMetadataTool {
    /// The flake to show outputs for.
    ///
    /// Examples: "github:neuro-soup/evochi", "/path/to/nixos/flake/dir", etc.
    flake: String,
}

impl FlakesMetadataTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        // Run: nix flake metadata --json <flake>
        let output = Command::new("nix")
            .args([
                "--extra-experimental-features",
                "nix-command flakes",
                "flake",
                "metadata",
                "--json",
                self.flake.as_str(),
            ])
            .output()
            .map_err(CallToolError::new)?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let err = Error::other(format!(
                r#"nix flake metadata (status: {}): {}"#,
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
    name = "wiki_search",
    description = "Search the NixOS wiki for pages matching a given regex."
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct WikiSearchTool {
    /// The name of the page to read from the NixOS wiki.
    ///
    /// Examples: "Docker", "Go", "Rust", etc.
    ///
    /// The resulting `title` can be passed as `name_of_the_found_page` to the
    /// `wiki_get_page` tool to read the page content.
    query: String,
}

impl WikiSearchTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        // GET https://wiki.nixos.org/w/api.php?action=query&list=search&srsearch=<query>&format=json
        let resp = ureq::get("https://wiki.nixos.org/w/api.php")
            .query("action", "query")
            .query("list", "search")
            .query("srsearch", &self.query)
            .query("format", "json")
            .call()
            .map_err(CallToolError::new)?;

        let body = resp.into_string().map_err(CallToolError::new)?;
        Ok(CallToolResult::text_content(vec![TextContent::from(body)]))
    }
}

#[mcp_tool(
    name = "wiki_get_page",
    description = "Read the page from NixOS's wiki."
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct WikiGetPageTool {
    /// The name of the page to read from the NixOS wiki.
    /// Prefer to search for single words, like "Rust", "Traefik", ..., and not
    /// "ACME Traefik".
    ///
    /// Examples: "Docker", "Go", "Rust", etc.
    title: String,
}

impl WikiGetPageTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        // GET https://wiki.nixos.org/w/rest.php/v1/page/<title>

        fn encode_title_for_path(title: &str) -> String {
            // Encode the title for use in a path segment. MediaWiki treats spaces
            // as underscores in titles, so normalize spaces to underscores and
            // percent-encode any reserved characters.

            let mut out = String::with_capacity(title.len());
            for &b in title.replace(' ', "_").as_bytes() {
                let is_unreserved =
                    b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'~');
                if is_unreserved {
                    out.push(b as char);
                } else {
                    // Percent-encode all other bytes.
                    out.push('%');
                    out.push_str(&format!("{:02X}", b));
                }
            }
            out
        }

        let encoded_title = encode_title_for_path(&self.title);
        let url = format!(
            "https://wiki.nixos.org/w/rest.php/v1/page/{}",
            encoded_title
        );

        let resp = ureq::get(&url).call().map_err(CallToolError::new)?;
        let status = resp.status();
        let status_text = resp.status_text().to_string();
        let body = resp.into_string().map_err(CallToolError::new)?;

        match serde_json::from_str::<serde_json::Value>(&body) {
            Ok(val) => {
                if let Some(src) = val.get("source").and_then(|v| v.as_str()) {
                    Ok(CallToolResult::text_content(vec![TextContent::from(
                        src.to_string(),
                    )]))
                } else {
                    let pretty = serde_json::to_string_pretty(&val).map_err(CallToolError::new)?;
                    let err = Error::other(format!(
                        "wiki returned JSON without 'source' field for title '{}'. Response: {}",
                        self.title, pretty
                    ));
                    Err(CallToolError::new(err))
                }
            }
            Err(e) => {
                let preview = if body.len() > 500 {
                    &body[..500]
                } else {
                    &body
                };
                let err = Error::other(format!(
                    "wiki returned invalid json (status {} {}): {}. Body preview: {}",
                    status, status_text, e, preview
                ));
                Err(CallToolError::new(err))
            }
        }
    }
}

#[mcp_tool(
    name = "config_check",
    description = "Check your system for potential problems and print a PASS or FAIL for each check."
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct ConfigCheckTool {}

impl ConfigCheckTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        // Run: nix config check --json <flake>
        let output = Command::new("nix")
            .args([
                "--extra-experimental-features",
                "nix-command flakes",
                "config",
                "check",
            ])
            .output()
            .map_err(CallToolError::new)?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let err = Error::other(format!(
                r#"nix config check (status: {}): {}"#,
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

#[mcp_tool(name = "config_show", description = "Show the Nix configuration.")]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct ConfigShowTool {}

impl ConfigShowTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        // Run: nix config show
        let output = Command::new("nix")
            .args([
                "--extra-experimental-features",
                "nix-command flakes",
                "config",
                "show",
            ])
            .output()
            .map_err(CallToolError::new)?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let err = Error::other(format!(
                r#"nix config show (status: {}): {}"#,
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
    name = "manix_search",
    description = "Search the documentation for a given query using manix."
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct ManixSearchTool {
    /// The query to search for in the documentation. The query represents the
    /// prefix of the options that you want to search for.
    ///
    /// Examples: "programs.git", "services.nginx", etc.
    query: String,
}

impl ManixSearchTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        // Run: nix run nixpkgs#manix -- <query>
        let output = Command::new("nix")
            .args([
                "--extra-experimental-features",
                "nix-command flakes",
                "run",
                "nixpkgs#manix",
                "--",
                self.query.as_str(),
            ])
            .output()
            .map_err(CallToolError::new)?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let err = Error::other(format!(
                r#"manix failed (status: {}): {}"#,
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
    name = "nix_manual_list",
    description = "List Markdown files in the Nix manual source directory."
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct NixManualListTool {}

impl NixManualListTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let tree_url = "https://api.github.com/repos/NixOS/nix/git/trees/master?recursive=1";
        let tree_resp = ureq::get(tree_url)
            .set(
                "User-Agent",
                "rime-manual-list/1.0 (+https://github.com/lukasl-dev/rime)",
            )
            .call()
            .map_err(CallToolError::new)?;

        let status = tree_resp.status();
        let status_text = tree_resp.status_text().to_string();
        let tree_body = tree_resp.into_string().map_err(CallToolError::new)?;

        let tree_json: serde_json::Value = match serde_json::from_str(&tree_body) {
            Ok(v) => v,
            Err(e) => {
                let preview = if tree_body.len() > 500 {
                    &tree_body[..500]
                } else {
                    &tree_body
                };
                let err = Error::other(format!(
                    "GitHub trees API returned invalid json (status {} {}): {}. Body preview: {}",
                    status, status_text, e, preview
                ));
                return Err(CallToolError::new(err));
            }
        };

        let Some(items) = tree_json.get("tree").and_then(|v| v.as_array()) else {
            let pretty =
                serde_json::to_string_pretty(&tree_json).unwrap_or_else(|_| tree_body.clone());
            let err = Error::other(format!(
                "GitHub trees API response missing 'tree' array. Response: {}",
                pretty
            ));
            return Err(CallToolError::new(err));
        };

        let prefix = "doc/manual/source/";
        let mut md_files: Vec<String> = items
            .iter()
            .filter_map(|item| {
                let path = item.get("path")?.as_str()?;
                let kind = item.get("type")?.as_str()?;
                if kind == "blob" && path.starts_with(prefix) && path.ends_with(".md") {
                    // Strip prefix and extension
                    let without_prefix = &path[prefix.len()..];
                    let without_ext = without_prefix.strip_suffix(".md").unwrap_or(without_prefix);
                    Some(without_ext.to_string())
                } else {
                    None
                }
            })
            .collect();

        md_files.sort();
        md_files.dedup();

        let pretty = serde_json::to_string_pretty(&md_files).map_err(CallToolError::new)?;
        Ok(CallToolResult::text_content(vec![TextContent::from(
            pretty,
        )]))
    }
}

#[mcp_tool(
    name = "nix_manual_read",
    description = "Read a specific Markdown file from the Nix manual source."
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct NixManualReadTool {
    /// The path to the file without the .md extension.
    ///
    /// Examples: "language/identifiers", "builtins", etc.
    path: String,
}

impl NixManualReadTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        let url = format!(
            "https://raw.githubusercontent.com/NixOS/nix/master/doc/manual/source/{}.md",
            self.path
        );

        let resp = ureq::get(&url).call().map_err(CallToolError::new)?;
        let body = resp.into_string().map_err(CallToolError::new)?;

        Ok(CallToolResult::text_content(vec![TextContent::from(body)]))
    }
}

tool_box!(
    RimeTools,
    [
        EvaluateTool,
        LogTool,
        PackagesSearchTool,
        PackagesWhyDepends,
        FlakesShowTool,
        FlakesMetadataTool,
        WikiSearchTool,
        WikiGetPageTool,
        ConfigCheckTool,
        ConfigShowTool,
        ManixSearchTool,
        NixManualListTool,
        NixManualReadTool,
    ]
);
