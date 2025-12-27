# rime

<div align="center">
    <a href="https://www.rust-lang.org/">
        <img src="https://img.shields.io/badge/Written_In-Rust-ce412b?style=for-the-badge&logo=rust" alt="Rust" />
    </a>
    <a href="https://nixos.org">
        <img src="https://img.shields.io/badge/MCP_for-NixOS-7ebae4?style=for-the-badge&logo=nixos" alt="NixOS" />
    </a>
</div>

<br>

A minimal [Model Context Protocol (MCP)](https://modelcontextprotocol.io) server for Nix tooling, written in Rust.

## Tools

<details>
<summary><b>❄️ Nix</b></summary>

- `nix_evaluate`: Evaluate a Nix expression.
- `nix_log`: Get build log for an installable.
- `nix_packages_search`: Search packages in an installable.
- `nix_packages_why_depends`: Show why a package depends on another.
- `nix_flakes_show`: Show a flake's outputs.
- `nix_flakes_metadata`: Show flake metadata.
- `nix_config_check`: Run `nix config check`.
- `nix_config_show`: Run `nix config show`.
- `nix_manual_list`: List Markdown files in the Nix manual.
- `nix_manual_read`: Read a Markdown file from the Nix manual.
- `nixhub_package_versions`: Get version history for a package via [nixhub](https://nixhub.io).
</details>

<details>
<summary><b>❄️ NixOS</b></summary>

- `nixos_wiki_search`: Search the NixOS wiki.
- `nixos_wiki_read_page`: Read a page from the NixOS wiki.
- `nixos_channels`: List available NixOS channels with their status.
</details>

<details>
<summary><b>🏠 Home Manager</b></summary>

- `home_manager_options_search`: Search Home Manager options.
</details>

<details>
<summary><b>🌑 nvf</b></summary>

- `nvf_options_search`: Search [nvf](https://github.com/notashelf/nvf) options.
- `nvf_manual_list`: List files in the nvf manual.
- `nvf_manual_read`: Read a file from the nvf manual.
</details>

<details>
<summary><b>🔍 General Tools</b></summary>

- `manix_search`: Search docs with [manix](https://github.com/mlvzk/manix).
</details>

## Getting Started

### Prerequisites

Ensure `nix` is installed and available on your `PATH`.

### Build & Run

```bash
# Build
nix build .#rime

# Run (Stdio)
./result/bin/rime stdio

# Run (HTTP)
./result/bin/rime http --host 127.0.0.1 --port 8080
```

## Usage

<details>
<summary><b>OpenAI Codex</b></summary>

Add to `~/.codex/config.toml`:

```toml
[mcp_servers.rime]
command = "/path/to/rime"
args = ["stdio"]
```
</details>

<details>
<summary><b>opencode</b></summary>

Add to `~/.config/opencode/opencode.json`:

```json
{
  "mcp": {
    "rime": {
      "type": "local",
      "command": ["/path/to/rime", "stdio"],
      "enabled": true
    }
  }
}
```
</details>

<details>
<summary><b>Claude Code</b></summary>

```bash
claude mcp add rime -- /path/to/rime stdio
```
</details>

<details>
<summary><b>Gemini Code</b></summary>

Add to `~/.gemini/settings.json`:

```json
{
  "mcpServers": {
    "rime": {
      "command": "/path/to/rime",
      "args": ["stdio"]
    }
  }
}
```
</details>

<details>
<summary><b>VSCode</b></summary>

Add to `.vscode/mcp.json`:

```json
{
  "servers": {
    "rime": {
      "command": "/path/to/rime",
      "args": ["stdio"]
    }
  }
}
```
</details>

<details>
<summary><b>Zed</b></summary>

Add to `settings.json`:

```json
{
  "context_servers": {
    "rime": {
      "command": "/path/to/rime",
      "args": ["stdio"]
    }
  }
}
```
</details>

## Credits

- [manix](https://github.com/mlvzk/manix)
- [nixhub](https://nixhub.io/)
- [nvf](https://github.com/notashelf/nvf)
- [mcp-nixos](https://github.com/utensils/mcp-nixos)
- [nvf](https://github.com/notashelf/nvf)
