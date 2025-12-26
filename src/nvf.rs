use serde::Deserialize;
use std::io::Error;
use std::process::Command;

#[derive(Debug, Deserialize)]
pub(crate) struct NvfOption {
    pub(crate) name: String,
    pub(crate) description: String,
    #[serde(rename = "type")]
    pub(crate) r#type: String,
    pub(crate) default: String,
}

pub(crate) fn search_nvf_options(query: &str) -> Result<Vec<NvfOption>, Error> {
    let expression = format!(
        r#"
let
  flake = builtins.getFlake "github:NotAShelf/nvf";
  pkgs = import <nixpkgs> {{}};
  eval = flake.lib.neovimConfiguration {{ inherit pkgs; modules = []; }};
  optionsList = pkgs.lib.optionAttrSetToDocList eval.options;
  query = "{}";
  results = builtins.filter (opt: pkgs.lib.hasInfix query opt.name) optionsList;
in
  builtins.map (opt: {{
    name = opt.name;
    description = if opt ? description then opt.description else "";
    type = if opt ? type then opt.type else "";
    default = if opt ? default then (if builtins.isAttrs opt.default && opt.default ? text then opt.default.text else builtins.toJSON opt.default) else "";
  }}) (pkgs.lib.take 20 results)
"#,
        query
    );

    let output = Command::new("nix")
        .args([
            "--extra-experimental-features",
            "nix-command flakes",
            "eval",
            "--json",
            "--impure",
            "--expr",
            &expression,
        ])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(Error::other(format!(
            "nix eval for nvf options failed: {}",
            stderr
        )));
    }

    let stdout = String::from_utf8(output.stdout)
        .map_err(|e| Error::other(format!("failed to read nix output: {}", e)))?;

    let options: Vec<NvfOption> = serde_json::from_str(&stdout)
        .map_err(|e| Error::other(format!("failed to parse nix output: {}", e)))?;

    Ok(options)
}

pub(crate) fn list_nvf_manual() -> Result<Vec<String>, Error> {
    let tree_url = "https://api.github.com/repos/NotAShelf/nvf/git/trees/main?recursive=1";
    let tree_resp = ureq::get(tree_url)
        .set(
            "User-Agent",
            "rime/1.0 (+https://github.com/lukasl-dev/rime)",
        )
        .call()
        .map_err(|e| Error::other(format!("GitHub API request failed: {}", e)))?;

    let tree_body = tree_resp
        .into_string()
        .map_err(|e| Error::other(format!("failed to read GitHub response: {}", e)))?;
    let tree_json: serde_json::Value = serde_json::from_str(&tree_body)
        .map_err(|e| Error::other(format!("failed to parse GitHub response: {}", e)))?;

    let Some(items) = tree_json.get("tree").and_then(|v| v.as_array()) else {
        return Err(Error::other("GitHub API response missing 'tree' array"));
    };

    let prefix = "docs/manual/";
    let mut md_files: Vec<String> = items
        .iter()
        .filter_map(|item| {
            let path = item.get("path")?.as_str()?;
            let kind = item.get("type")?.as_str()?;
            if kind == "blob" && path.starts_with(prefix) && path.ends_with(".md") {
                let without_prefix = &path[prefix.len()..];
                let without_ext = without_prefix.strip_suffix(".md").unwrap_or(without_prefix);
                Some(without_ext.to_string())
            } else {
                None
            }
        })
        .collect();

    md_files.sort();
    Ok(md_files)
}

pub(crate) fn read_nvf_manual(path: &str) -> Result<String, Error> {
    let url = format!(
        "https://raw.githubusercontent.com/NotAShelf/nvf/main/docs/manual/{}.md",
        path
    );

    let resp = ureq::get(&url)
        .call()
        .map_err(|e| Error::other(format!("failed to fetch nvf manual: {}", e)))?;
    let body = resp
        .into_string()
        .map_err(|e| Error::other(format!("failed to read nvf manual: {}", e)))?;

    Ok(body)
}
