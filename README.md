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

Minimal MCP server for Nix tooling, written in Rust. It uses the [rust-mcp-sdk](https://github.com/rust-mcp-stack/rust-mcp-sdk)
and exposes a set of helpful Nix/NixOS tools through the [Model Context Protocol (MCP)](https://modelcontextprotocol.io).

The binary can run as either:

- MCP over stdio
- MCP over HTTP (with SSE or JSON-only responses)

## Tools

- `nix_evaluate`: Evaluate a Nix expression.
- `nix_log`: Get build log for an installable.
- `nix_packages_search`: Search packages in an installable.
- `nix_packages_why_depends`: Show why a package depends on another.
- `nix_flakes_show`: Show a flake's outputs.
- `nix_flakes_metadata`: Show flake metadata.
- `nix_config_check`: Run `nix config check`.
- `nix_config_show`: Run `nix config show`.
- `nix_manual_list`: List Markdown files in the Nix manual source.
- `nix_manual_read`: Read a Markdown file from the Nix manual.
- `nixos_wiki_search`: Search the NixOS wiki.
- `nixos_wiki_read_page`: Read a page from the NixOS wiki.
- `manix_search`: Search docs with [manix](https://github.com/mlvzk/manix).

Note: Most tools shell out to `nix`; ensure `nix` is installed and available on `PATH`.

## Build

With Cargo:

- Debug: `cargo build`
- Release: `cargo build --release`

With Nix (flake):

- Build: `nix build .#rime` (binary at `./result/bin/rime`)

## Run

Help:

- Cargo: `cargo run -- --help`
- Nix: `nix run .#rime -- --help`

Stdio transport:

- Cargo: `cargo run -- stdio`
- Nix: `nix run .#rime -- stdio`

HTTP transport:

- Cargo: `cargo run -- http --host 127.0.0.1 --port 8080`
- Nix: `nix run .#rime -- http --host 127.0.0.1 --port 8080`

## Usage

### OpenAI Codex

Add the following snippet into your `~/.codex/config.toml`:

```toml
[mcp_servers.rime]
command = "/path/to/rime"
args = ["stdio"]
```

[See codex docs.](https://github.com/openai/codex?tab=readme-ov-file#model-context-protocol-mcp)

### opencode

Add a new MCP server into your opencode config, e.g. globally at `~/.config/opencode/opencode.json`:

```json
{
  "$schema": "https://opencode.ai/config.json",
  "mcp": {
    "rime": {
      "type": "local",
      "command": ["/path/to/rime", "stdio"],
      "enabled": true
    }
  }
}
```

[See opencode docs.](https://opencode.ai/docs/mcp-servers)

### Claude Code

Run the following command:

```bash
claude mcp add rime -- /path/to/rime stdio
```

[See Claude Code docs.](https://docs.anthropic.com/en/docs/claude-code/mcp#local-scope)

### Gemini Code

Add the following `rime` to `mcpServers` in your Gemini config, e.g. globally at `~/.gemini/settings.json`.

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

[See Gemini Code docs.](https://github.com/google-gemini/gemini-cli/blob/main/docs/tools/mcp-server.md#how-to-set-up-your-mcp-server)

### VSCode

Add `rime` to `.vscode/mcp.json`:

```json
{
  "servers": {
    "rime": {
      "type": "stdio",
      "command": "/path/to/rime",
      "args": ["stdio"]
    }
  }
}
```

[See VSCode Copilot docs.](https://code.visualstudio.com/docs/copilot/chat/mcp-servers#_add-an-mcp-server)
