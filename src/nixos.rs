use serde::Deserialize;
use std::io::Error;
use std::process::Command;

#[derive(Debug, Deserialize)]
pub(crate) struct NixosOption {
    pub(crate) name: String,
    pub(crate) description: String,
    #[serde(rename = "type")]
    pub(crate) r#type: String,
    pub(crate) default: String,
}

pub(crate) fn search_nixos_options(query: &str, ref_name: &str) -> Result<Vec<NixosOption>, Error> {
    let expression = format!(
        r#"
let
  nixpkgs = builtins.getFlake "github:NixOS/nixpkgs/{}";
  pkgs = import nixpkgs {{}};
  eval = import "${{nixpkgs}}/nixos/lib/eval-config.nix" {{
    inherit pkgs;
    modules = [];
  }};
  optionsList = pkgs.lib.optionAttrSetToDocList eval.options;
  query = "{}";
  results = builtins.filter (opt: pkgs.lib.hasInfix query opt.name) optionsList;
in
  builtins.map (opt: {{
    name = opt.name;
    description = if opt ? description then (if builtins.isAttrs opt.description && opt.description ? text then opt.description.text else if builtins.isString opt.description then opt.description else "") else "";
    type = if opt ? type then (if builtins.isString opt.type then opt.type else if builtins.isAttrs opt.type && opt.type ? description then opt.type.description else "") else "";
    default = if opt ? default then (if builtins.isAttrs opt.default && opt.default ? text then opt.default.text else builtins.toJSON opt.default) else "";
  }}) (pkgs.lib.take 20 results)
"#,
        ref_name, query
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
            "nix eval for nixos options failed: {}",
            stderr
        )));
    }

    let stdout = String::from_utf8(output.stdout)
        .map_err(|e| Error::other(format!("failed to read nix output: {}", e)))?;

    let options: Vec<NixosOption> = serde_json::from_str(&stdout)
        .map_err(|e| Error::other(format!("failed to parse nix output: {}", e)))?;

    Ok(options)
}
