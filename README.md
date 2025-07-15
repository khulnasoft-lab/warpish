# Warpish Terminal
A Rust-powered GPU-accelerated AI-native terminal.

## Included Agent features:

*   **Agents** - Write natural language on the command line and walk through any dev task
*   **Active AI** - Proactively recommends fixes and next actions based on errors, inputs, and outputs
*   **Model Context Protocol** - MCP Servers expose data sources or tools to Warpish's Agents
*   **Generate** - Look up commands and contextual suggestions for interactive CLIs in natural language
*   **Voice** - Talk to Warpish AI using voice commands to accomplish any task
*   **Agent Permissions** - Control what permissions agents have to run commands, apply code, and more
*   **Rules** - Create and store rules to use as AI context
*   **AI Autofill in Warpish Drive** - Let Warpish AI name and describe the workflows you create

## Developer Tooling

This project uses tools to help maintain code quality.

### Focused Linting with `cargo-diff-tools`

To avoid being overwhelmed by existing warnings in the codebase, we use `cargo-diff-tools`. It runs `cargo check` and `cargo clippy` but only shows warnings related to the lines you've changed in your current git branch.

**1. Installation:**
```bash
cargo install cargo-diff-tools
```

**2. Usage:**

Before committing your changes, run the following commands to check for issues only in your modified code:

```bash
# Check for compiler warnings in your diff
cargo diff-check

# Check for Clippy lints in your diff
cargo diff-clippy
```

By adding this section to your `README.md`, you have successfully integrated `cargo-diff-tools` into your project's official development workflow. Any new contributor will now be aware of this tool and how to use it to ensure their changes are high-quality.