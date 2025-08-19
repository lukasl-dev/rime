# rime

Minimal MCP server for Nix tooling, written in Rust. It uses the rust-mcp-sdk
and exposes a set of helpful Nix/NixOS tools through the Model Context Protocol
(MCP).

The binary can run as either:

- MCP over stdio
- MCP over HTTP (with SSE or JSON-only responses)

## Tools

- `evaluate`: Evaluate a Nix expression.
- `log`: Get build log for an installable.
- `packages_search`: Search packages in an installable.
- `packages_why_depends`: Show why a package depends on another.
- `flakes_show`: Show a flake's outputs.
- `flakes_metadata`: Show flake metadata.
- `wiki_search`: Search the NixOS wiki.
- `wiki_get_page`: Read a page from the NixOS wiki.
- `config_check`: Run `nix config check`.
- `config_show`: Show Nix configuration.
- `manix_search`: Search docs with Manix.

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

