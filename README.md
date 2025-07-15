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

## Project Cleanup & Fixes (Recent Update)
This project has undergone a significant cleanup to improve maintainability, portability, and correctness. Key changes include:
- **Fixed `Cargo.toml`:** The dependency list was streamlined to include only direct dependencies, removing hundreds of redundant transitive entries.
- **Fixed `diesel.toml`:** Corrected a hardcoded, absolute file path to a relative one, making the project buildable on any machine.
- **Corrected YAML Syntax:** Invalid syntax in `themes/baitong.yaml` was fixed.
- **Resolved Compiler Errors:** A critical "unclosed delimiter" error in `src/ui/renderer.rs` was fixed by adding the missing brace and reformatting the code for readability.
- **Streamlined Scripts:** The `bootstrap` script was updated to use the standard `crates.io` for installing tools.

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

By adding this section to your README.md, you have successfully integrated cargo-diff-tools into your project's official development workflow. Any new contributor will now be aware of this tool and how to use it to ensure their changes are high-quality.