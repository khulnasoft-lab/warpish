// Core application modules
pub mod app;
pub mod ui;
pub mod agent;
pub mod pty;
pub mod config;
pub mod error;
pub mod event;
pub mod completions;
pub mod completions_ui;

// Editor and input modules
pub mod vim;
pub mod keybindings;

// Data and persistence modules
pub mod db;
pub mod drive;
pub mod session;

// AI and language processing modules
pub mod agent_mode_eval;
pub mod natural_language_detection;
pub mod mcq;

// Content processing modules
pub mod markdown_parser;
pub mod syntax_parser;
pub mod languages;
pub mod code;

// System and utility modules
pub mod assets;
pub mod asset_macro;
pub mod command;
pub mod fuzzy_match;
pub mod string_offset;
pub mod sum_tree;
pub mod syntax_tree;
pub mod virtual_fs;
pub mod watcher;
pub mod scripting;

// Network and communication modules
pub mod websocket;
pub mod graphql;
pub mod serve_wasm;
pub mod lpc;

// Integration and resources
pub mod integration;
pub mod resources;
pub mod rules;
