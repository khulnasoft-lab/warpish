# TODO - Warpish Terminal v2

## 🚨 Immediate Actions

* [ ] **Resolve Critical Build Issues**: Focus on core compilation and runtime stability.
    * [ ] Fix any remaining build errors on the `main` branch.
    * [ ] Restore broken theme loading functionality.
    * [ ] Investigate and resolve crashes occurring on startup.

---

## High Priority (Critical)

### 🔥 Build & Compilation

* [ ] **Clean Up & Modernize Codebase**:
    * [ ] Eliminate all remaining unused import warnings.
    * [ ] Address any remaining deprecated API usage warnings.
    * [ ] Ensure all dependencies in `Cargo.toml` are up to date and compatible.
    * [ ] Replace all `unwrap()` calls with proper error handling mechanisms.
* [ ] **Cross-Platform Compilation Verification**:
    * [ ] Test compilation on Linux.
    * [ ] Test compilation on Windows.
    * [ ] Test compilation on macOS.

### 🎯 Core Functionality

* [ ] **Terminal Rendering & Them
    ing**:
    * [ ] Verify accurate terminal rendering post-theme fixes.
    * [ ] Ensure `colors` field in themes is correctly applied and renders as expected.
    * [ ] Test color rendering across different terminal environments.
* [ ] **Pane & Input Management**:
    * [ ] Thoroughly test pane lifecycle: creation, switching, and closing.
    * [ ] Validate VTE handling with the simplified grid implementation.
    * [ ] Confirm keyboard input handling functions correctly.
* [ ] **UI Renderer & Buffer Logic**:
    * [ ] Implement proper VTE synchronization within `src/ui/renderer.rs`.
    * [ ] Implement robust buffer resizing logic in `src/ui/renderer.rs`.
    * [ ] Verify and correct the field used for glyph width calculation in `src/main.rs` (font metrics).

---

## Medium Priority (Important)

### 📚 Documentation

* [ ] **API & Internal Documentation**:
    * [ ] Document the `App` struct and its primary components.
    * [ ] Add comprehensive module-level documentation for:
        * [ ] `app/state.rs` (Application state management)
        * [ ] `pty/vte_handler.rs` (Terminal emulation handling)
        * [ ] `ui/terminal_ui.rs` (User interface components)
        * [ ] `app/pane_manager.rs` (Pane management system)
        * [ ] `agent/client.rs` (AI agent integration)
        * [ ] `mcq/mod.rs` (Multiple Choice Question handling)
    * [ ] Generate API documentation using `cargo doc`.
    * [ ] Update inline code comments for improved clarity and understanding.

### 🔧 Code Quality & Testing

* [ ] **Refactoring & Error Handling**:
    * [ ] Refactor large functions in `terminal_ui.rs` into smaller, more focused units.
    * [ ] Implement specific error types instead of generic `Box<dyn Error>`.
* [ ] **Testing Suite Expansion**:
    * [ ] Add comprehensive unit tests for core modules.
    * [ ] Implement integration tests for key terminal functionalities.
* [ ] **Best Practices & Optimization**:
    * [ ] Apply `clippy` lint fixes and adhere to Rust best practices.
    * [ ] Review and optimize memory usage patterns throughout the application.

### 🎨 UI/UX Improvements

* [ ] **Interactive Features**:
    * [ ] Implement proper resize handling for terminal windows.
    * [ ] Add smooth scrolling and cursor movement.
    * [ ] Enhance text selection and copy/paste functionality.
* [ ] **Content Rendering**:
    * [ ] Add syntax highlighting support for markdown code block rendering (`src/markdown_parser/renderer.rs`).

### 🤖 AI Agent Integration

* [ ] **Agent Reliability**:
    * [ ] Audit agent client authentication and API call security.
    * [ ] Thoroughly test agent response handling, including error cases.
    * [ ] Implement proper timeout handling for all agent requests.
* [ ] **Agent Configuration & Workflow**:
    * [ ] Add configuration options to control agent behavior.
    * [ ] Test the Multiple Choice Question (MCQ) workflow end-to-end.

---

## Low Priority (Nice to Have)

### 📝 Configuration & Settings

* [ ] Review and validate the `config.toml` structure.
* [ ] Implement runtime configuration reloading without restart.
* [ ] Ensure user preferences are persistently stored.
* [ ] Add customization options for keybindings.
* [ ] Create default configuration templates for users.

### 🧪 Testing & Quality Assurance

* [ ] Set up a continuous integration (CI) pipeline.
* [ ] Add benchmark tests for performance-critical code sections.
* [ ] Create test fixtures for various common terminal scenarios.
* [ ] Implement property-based testing for VTE handling.
* [ ] Add fuzzing tests for input handling robustness.

### 📋 File Management

* [ ] Implement proper file watching with auto-reload capabilities.
* [ ] Add a file tree navigation interface.
* [ ] Implement search functionality across open files/buffers.
* [ ] Add syntax highlighting for different file types.
* [ ] Create backup and recovery mechanisms for sessions/data.

### 🔌 Extensions & Plugins

* [ ] Design a robust plugin architecture.
* [ ] Implement Lua scripting improvements in `scripting/mod.rs`.
* [ ] Add support for custom terminal commands.
* [ ] Create comprehensive extension API documentation.
* [ ] Implement plugin discovery and loading mechanisms.

### 🌐 Network & Connectivity

* [ ] Test network connectivity for all agent features.
* [ ] Implement proper SSL/TLS certificate handling.
* [ ] Add proxy support for all network requests.
* [ ] Implement retry logic for failed network connections.

---

## Technical Debt

### 🔄 Refactoring Needed

* [ ] Simplify overly complex `match` statements in the VTE handler.
* [ ] Extract common code patterns into reusable utility functions.
* [ ] Reduce code duplication across UI modules.
* [ ] Improve separation of concerns between modules for better maintainability.
* [ ] Standardize error handling patterns across the entire codebase.

### 🏗️ Architecture Improvements

* [ ] Implement a proper event system for inter-component communication.
* [ ] Add robust lifecycle management for application resources.
* [ ] Create clear abstraction layers for platform-specific code.
* [ ] Implement proper dependency injection where beneficial.
* [ ] Add graceful shutdown handling for clean application exit.

---

## Documentation Tasks

### 📖 User Documentation

* [ ] Update `README.md` with current features, installation, and basic usage.
* [ ] Create a comprehensive user guide, including screenshots.
* [ ] Add a dedicated troubleshooting section.
* [ ] Document all configuration options clearly.
* [ ] Create detailed contribution guidelines for potential contributors.

### 👨‍💻 Developer Documentation

* [ ] Document the build process and all necessary dependencies.
* [ ] Create an overview of the application's architecture.
* [ ] Add clear code style guidelines.
* [ ] Document all testing procedures.
* [ ] Create deployment instructions.

---

## Future Enhancements

### 🚀 Feature Additions

* [ ] Add tab support for managing multiple terminal sessions.
* [ ] Implement split-screen functionality for enhanced multi-tasking.
* [ ] Add terminal recording and playback capabilities.
* [ ] Allow creation of custom prompt themes.
* [ ] Implement a robust notification system.

### 🔮 Advanced Features

* [ ] Implement terminal multiplexing (e.g., similar to tmux/screen).
* [ ] Add remote terminal support.
* [ ] Create terminal sharing capabilities for collaboration.
* [ ] Add advanced search and filtering within terminal output.
* [ ] Implement terminal automation scripting (e.g., macros).

---

## Progress Tracking

### ✅ Completed (Recent Fixes)

* [x] Fixed compilation errors (45 errors resolved)
* [x] Corrected duplicate imports
* [x] Fixed theme color field access (`terminal_colors` → `colors`)
* [x] Resolved VTE handler implementation
* [x] Fixed pane manager access patterns
* [x] Corrected `cosmic_text` API usage
* [x] Fixed async/await patterns in completions
* [x] Resolved borrowing issues in agent client
* [x] Fixed syntax errors (missing braces)
* [x] Cleaned up major unused imports

### 📦 Module Status

| Module | Status | Phase | Priority |
| :------------------------- | :------------ | :-------------- | :------- |
| watcher | ⬜ Not started | Planning | Medium |
| websocket | ⬜ Not started | Planning | Medium |
| virtual_fs | ⬜ Not started | Planning | Medium |
| warpish_drive | ⬜ Not started | Planning | Medium |
| syntax_tree | ⬜ Not started | Planning | Medium |
| ui | ⬜ Not started | Planning | Medium |
| string_offset | ⬜ Not started | Planning | Medium |
| session | ⬜ Not started | Planning | Medium |
| serve_wasm | ⬜ Not started | Planning | Medium |
| scripting | ⬜ Not started | Planning | Medium |
| rules | ⬜ Not started | Planning | Medium |
| resources | ⬜ Not started | Planning | Medium |
| renderer | ⬜ Not started | Planning | Medium |
| render | ⬜ Not started | Planning | Medium |
| natural_language_detection | ⬜ Not started | Planning | Medium |
| mcq | ⬜ Not started | Planning | Medium |
| markdown_parser | ⬜ Not started | Planning | Medium |
| lpc | ⬜ Not started | Planning | Medium |
| languages | ⬜ Not started | Planning | Medium |
| integration | ⬜ Not started | Planning | Medium |
| graphql | ⬜ Not started | Planning | Medium |
| fuzzy_match | ⬜ Not started | Planning | Medium |
| features | ⬜ Not started | Planning | Medium |
| data | ⬜ Not started | Planning | Medium |
| config | ⬜ Not started | Planning | Medium |
| command | ⬜ Not started | Planning | Medium |
| code | ⬜ Not started | Planning | Medium |
| asset_macro | ⬜ Not started | Planning | Medium |
| ai | ⬜ Not started | Planning | Medium |
| agent_mode_eval | ⬜ Not started | Planning | Medium |
| agent | ⬜ Not started | Planning | Medium |

### 📊 Current Status

* **Build Status**: ✅ Compiles successfully
* **Warnings**: ⚠️ Some unused import warnings remain
* **Tests**: ❓ Need to be implemented/verified
* **Documentation**: 📝 Needs significant updates

---

## Notes

This TODO list is organized by priority and category. Focus on **High Priority** items first to ensure the application is stable and functional. **Medium Priority** items will significantly improve code quality and user experience. **Low Priority** items are enhancements that can be addressed over time.

Regular review and updates of this TODO list are highly recommended as the project evolves.

Last updated: July 15, 2025

---

### Key changes and rationale:

* **"Example:" Removed**: Since you're improving your actual `TODO.md`, the "Example:" prefixes are no longer necessary and make the tasks sound less like immediate actions.
* **More Action-Oriented Language**: I've rephrased some items to start with strong verbs, making them feel more like direct instructions (e.g., "Resolve Critical Build Issues" instead of "Fix build error").
* **Grouping Related Items**: For instance, "Test compilation on different platforms" was broken down into individual platforms to ensure each one is checked off explicitly. This makes the progress more granular and easier to track.
* **Clarity on `unwrap()`**: Explicitly stating "Replace all `unwrap()` calls with proper error handling mechanisms" is clearer than just "Add proper error handling for all `unwrap()` calls."
* **Bold Keywords**: I've bolded keywords and phrases that are particularly important or represent categories to make the document more scannable.
* **Added a "Key changes and rationale" section (for you)**: This helps explain *why* certain changes were made, which can be useful for future `TODO.md` improvements. This section wouldn't be in the final `TODO.md` you use.
* **Removed `$(date)`**: Replaced with a static date. While `$(date)` is useful for an automated script, in a manually maintained `TODO.md`, a fixed date is usually preferred unless you have a build system that updates it.
