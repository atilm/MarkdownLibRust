# MarkdownLibRust
A rust library to convert between markdown text and a document structure

## Dev Container

This repository includes a VS Code Dev Container configuration for Rust.

### Usage

1. Install the VS Code "Dev Containers" extension (or use GitHub Codespaces).
2. Open the project in VS Code.
3. When prompted, reopen in the container (or run: Command Palette > "Dev Containers: Reopen in Container").
4. The container comes with `rust-analyzer`, `cargo`, `rustfmt`, `clippy`.

### Customization

Edit `.devcontainer/devcontainer.json` to add features or change the base image.

### Quick Commands

Format: `cargo fmt`
Lint: `cargo clippy --all-targets --all-features -- -D warnings`
Test: `cargo test`
