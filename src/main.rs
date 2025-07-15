use anyhow::Result;
use arboard::Clipboard;
use chrono::Utc;
use cosmic_text::{
    Attrs, AttrsList, Buffer, Color, Editor, FontSystem, Metrics, Shaping, Underline,
};
use font_kit::{
    family_name::FamilyName,
    handle::Handle,
    properties::{Properties, Style, Weight},
    source::SystemSource,
};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers as GlobalModifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager,
};
use log::{error, info, warn};
use portable_pty::{CommandBuilder, NativePtySystem, PtySize};
use pprof::ProfilerGuard;
use rfd::FileDialog;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use shellwords;
use similar::{ChangeTag, TextDiff};
use std::fs;
use std::io::Write as _;
use std::path::Path;
use std::time::SystemTime;
use std::{
    io::{self, Read, Write},
    path::PathBuf,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use thiserror::Error;
use tokio::sync::mpsc;
use uuid::Uuid;
use warpish_terminal_v2::{
    agent::client::{AgentResponse, SimulatedAgent},
    app::{
        pane::Pane,
        state::{AgentState, App, AppMode, CursorShape, InputPosition, PaletteItem, PromptMode},
    },
    assets::Asset,
    completions::CompletionManager,
    completions_ui::CompletionsManager,
    config::{load_config, AppearanceConfig, Config},
    db::establish_connection,
    drive::{DriveManager, DriveObject, Notebook, Prompt, WorkflowBrowserState},
    error::AppError,
    event::UserAppEvent,
    input_handler::handle_input,
    keybindings::{load_keymap_from_yaml, KeyBinding, Keymap},
    pty::vte_handler::VteState,
    rules::{Rule, RuleAction},
    ui::{
        renderer::Renderer,
        theme::{load_theme, Theme, ThemeManager}, // Added load_theme here
    },
    vim::VimState,
    workflows::Workflow,
};
use winit::{
    event::{ElementState, Event, KeyEvent as WinitKeyEvent, Modifiers, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct EditorConfig {
    #[serde(default = "default_true")]
    pub autosuggestions: bool,
}

fn default_true() -> bool {
    true
}

pub fn main() -> Result<()> {
    env_logger::init();
    info!("Starting Warpish Terminal");

    let mut config = load_config().unwrap_or_default();

    // Load theme based on config
    let theme_path = config
        .appearance
        .theme
        .custom_theme_path
        .clone()
        .unwrap_or_else(|| format!("themes/{}.yaml", config.appearance.theme.name));
    let theme = load_theme(Path::new(&theme_path)).unwrap_or_default();

    let font_data = load_font(&config.appearance);

    let mut font_system = FontSystem::new();
    font_system.db_mut().load_font_data(font_data.clone());
    let metrics = Metrics::new(
        config.appearance.font_size,
        config.appearance.font_size * config.appearance.line_height,
    );
    let mut buffer_mono = Buffer::new(&mut font_system, metrics);
    buffer_mono.set_text(&mut font_system, "M", Attrs::new(), Shaping::Advanced);
    let char_width = buffer_mono
        .layout_runs()
        .next()
        .map_or(metrics.font_size, |run| run.glyph_w);
    let char_height = metrics.line_height;

    let initial_size = if config.appearance.window_size.use_custom_size {
        winit::dpi::PhysicalSize::new(
            (config.appearance.window_size.columns as f32 * char_width).ceil() as u32,
            (config.appearance.window_size.rows as f32 * char_height).ceil() as u32,
        )
    } else {
        winit::dpi::PhysicalSize::new(900, 600)
    };

    let event_loop: EventLoop<UserAppEvent> = EventLoop::with_user_event();
    let window = WindowBuilder::new()
        .with_title("Warpish Terminal")
        .with_inner_size(initial_size)
        .with_transparent(config.appearance.opacity < 1.0 || config.appearance.blur)
        .build(&event_loop)
        .unwrap();

    if config.appearance.blur {
        #[cfg(target_os = "macos")]
        {
            use winit::platform::macos::WindowExtMacOS;
            let _ = window.set_blur(true);
        }
        #[cfg(target_os = "windows")]
        {
            use winit::platform::windows::WindowExtWindows;
            let _ = window.set_acrylic(true);
        }
    }

    let mut db_conn = establish_connection();
    info!("Database connection established.");

    match crate::rules::load_rules_from_yaml(Path::new("rules.yaml")) {
        Ok(loaded_rules) => {
            info!(
                "Successfully loaded {} rules from rules.yaml.",
                loaded_rules.len()
            );
            for rule in loaded_rules {
                log::debug!("Loaded rule: '{}'", rule.name);
            }
        }
        Err(e) => {
            warn!("Could not load rules from rules.yaml: {}", e);
        }
    }

    let drive_manager = DriveManager::new().expect("Failed to initialize Warpish Drive");
    info!(
        "Drive loaded. Personal: {} objects, Teams: {} workspaces.",
        drive_manager.personal_ws.objects.len(),
        drive_manager.team_workspaces.len()
    );

    // Initialize completions system
    let mut completions_manager = CompletionsManager::new();
    completions_manager.is_enabled = config.editor.completions.enabled;
    completions_manager.trigger_chars = config.editor.completions.trigger_chars.clone();
    completions_manager.min_trigger_length = config.editor.completions.min_trigger_length;

    // Initialize Vim state if enabled (not used in the provided event loop, but kept for completeness)
    let mut vim_state = if config.editor.vim_enabled {
        Some(VimState::default())
    } else {
        None
    };

    let event_loop_proxy = event_loop.create_proxy();
    std::thread::spawn(move || {
        let receiver = GlobalHotKeyEvent::receiver();
        while let Ok(event) = receiver.recv() {
            event_loop_proxy
                .send_event(UserAppEvent::GlobalHotkey(event.id))
                .ok();
        }
    });

    let mut renderer = pollster::block_on(Renderer::new(&window, font_data, &config.appearance));
    let window_size = window.inner_size();
    let (grid_cols, grid_rows) = renderer.resize(window_size);

    let agent = SimulatedAgent::new(); // Initialize SimulatedAgent here

    let mut app = App::new(
        vec![Pane::new(
            grid_cols,
            grid_rows,
            &config.user.shell.clone().unwrap_or_else(|| {
                if cfg!(windows) {
                    "powershell.exe".to_string()
                } else {
                    "bash".to_string()
                }
            }),
            event_loop.create_proxy(),
        )],
        drive_manager,
        ThemeManager::new(),
        theme, // Use the loaded theme
        config.clone(),
        db_conn,
        completions_manager,
    );

    let tokio_runtime = tokio::runtime::Runtime::new().unwrap();
    let arc_completions_manager = Arc::new(Mutex::new(app.completions_manager.clone()));

    event_loop
        .run(move |event, elwt| {
            elwt.set_control_flow(ControlFlow::Wait);

            match event {
                Event::UserEvent(app_event) => match app_event {
                    UserAppEvent::PtyOutput => {
                        window.request_redraw();
                    }
                    UserAppEvent::AgentCompleted { pane_id, response } => {
                        if let Some(pane) = app.panes.iter_mut().find(|p| p.id == pane_id) {
                            if let Some(agent_state) = &mut pane.agent_state {
                                agent_state.status = crate::app::pane::AgentStatus::WaitingForInput;
                                agent_state.conversation.last_mut().unwrap().1 = response;
                            }
                        }
                    }
                    _ => {}
                },
                Event::WindowEvent { window_id, event } if window_id == window.id() => {
                    match event {
                        WindowEvent::CloseRequested => elwt.exit(),
                        WindowEvent::Resized(physical_size) => {
                            let (new_cols, new_rows) = renderer.resize(physical_size);
                            for pane in &mut app.panes {
                                pane.resize(new_cols, new_rows);
                            }
                            window.request_redraw();
                        }
                        WindowEvent::KeyboardInput { event: key, .. } => {
                            if let PhysicalKey::Code(key_code) = key.physical_key {
                                let active_pane = &mut app.panes[app.active_pane_idx];
                                match app.mode {
                                    AppMode::Agent(_) => {
                                        if key.state == ElementState::Pressed
                                            && key_code == KeyCode::Enter
                                        {
                                            if let Some(agent_state) = &mut active_pane.agent_state
                                            {
                                                let query = agent_state.current_input.clone();
                                                let model_to_use = agent_state.model_used.clone();
                                                let event_proxy = event_loop.create_proxy();
                                                let pane_id = active_pane.id;

                                                let agent_clone = agent.clone(); // Clone agent for async use
                                                tokio_runtime.spawn(async move {
                                                    let response = agent_clone.process_query(
                                                        &query,
                                                        &[],
                                                        &[],
                                                        model_to_use,
                                                    );
                                                    event_proxy
                                                        .send_event(UserAppEvent::AgentCompleted {
                                                            pane_id,
                                                            response,
                                                        })
                                                        .ok();
                                                });
                                            }
                                        }
                                    }
                                    AppMode::Normal => {
                                        let mut text_changed = false; // This needs to be set based on handle_input result
                                        let mut clipboard = Clipboard::new().unwrap_or_else(|_| {
                                            warn!("Failed to initialize clipboard");
                                            Clipboard::new().unwrap()
                                        });

                                        let input_result = app.handle_input(&key, &mut clipboard);

                                        // Check if input_result indicates text change (e.g., Some(command_string) implies text was processed)
                                        if input_result.is_some() {
                                            text_changed = true;
                                        }

                                        if let Some(input_text) = input_result {
                                            if !input_text.is_empty() {
                                                active_pane
                                                    .pty_writer
                                                    .write_all(input_text.as_bytes())
                                                    .unwrap();

                                                // Add to completions history
                                                arc_completions_manager
                                                    .lock()
                                                    .unwrap()
                                                    .add_to_history(input_text.clone());
                                            }
                                        }

                                        // Update completions if text changed
                                        if text_changed {
                                            let current_text = app
                                                .input_editor
                                                .buffer()
                                                .lines
                                                .iter()
                                                .map(|line| line.text())
                                                .collect::<String>();
                                            let cursor_pos =
                                                app.input_editor.buffer().cursor().index;

                                            // Spawn async task to update completions
                                            let completions_manager_clone =
                                                arc_completions_manager.clone();
                                            let current_text_clone = current_text.clone();
                                            tokio_runtime.spawn(async move {
                                                completions_manager_clone
                                                    .lock()
                                                    .unwrap()
                                                    .update_suggestions(
                                                        &current_text_clone,
                                                        cursor_pos,
                                                    )
                                                    .await;
                                            });
                                        }

                                        // Handle autosuggestions
                                        if config.editor.autosuggestions {
                                            app.update_autosuggestion(&mut db_conn);
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        WindowEvent::RedrawRequested => {
                            // Render the completions UI if visible
                            // This part might need more detailed integration within renderer.render
                            // For now, it just requests redraw, which is handled by the main render call.
                            if app.completions_manager.ui.is_visible {
                                // The main render call in renderer.render(&mut app, ...) should ideally
                                // handle rendering the completions UI based on app.completions_manager state.
                                window.request_redraw(); // Ensure a redraw happens
                            }

                            match renderer.render(&mut app, Duration::from_secs(0)) {
                                Ok(_) => {}
                                Err(wgpu::SurfaceError::Lost) => {
                                    renderer.resize(window.inner_size());
                                }
                                Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                                Err(e) => eprintln!("Error: {:?}", e),
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        })
        .unwrap();

    Ok(())
}

fn load_font(config: &AppearanceConfig) -> Vec<u8> {
    let source = SystemSource::new();
    let weight = match config.font_weight.to_lowercase().as_str() {
        "bold" => Weight::BOLD,
        "heavy" => Weight::HEAVY,
        "light" => Weight::LIGHT,
        _ => Weight::NORMAL,
    };
    let props = Properties::new().weight(weight).style(Style::Normal);
    let font_families = [
        FamilyName::Title(config.font_family.clone()),
        FamilyName::Title("Fira Code".into()),
        FamilyName::Title("Hack".into()),
        FamilyName::Title("Consolas".into()),
        FamilyName::Monospace,
    ];
    let best_match = font_families
        .iter()
        .find_map(|family| source.select_best_match(&[family.clone()], &props).ok());
    match best_match {
        Some(handle) => {
            info!(
                "Font found via font-kit: {:?}",
                handle.font().unwrap().full_name()
            );
            match handle {
                Handle::Path { path, .. } => std::fs::read(path).expect("Failed to read font file"),
                Handle::Memory { bytes, .. } => bytes.to_vec(),
            }
        }
        None => {
            warn!("No suitable system font found. Falling back to embedded JetBrains Mono.");
            let font_file = Asset::get("JetBrainsMono-Regular.ttf")
                .expect("Failed to load embedded fallback font.");
            font_file.data.into_owned()
        }
    }
}