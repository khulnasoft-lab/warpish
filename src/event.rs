use crossterm::event::{self, Event as CrosstermEvent, KeyEvent};
use futures::StreamExt;
use std::time::Duration;
use tokio::sync::mpsc;
use uuid::Uuid;
use crate::agent::client::AgentResponse;

/// Application events that drive state changes.
#[derive(Debug)]
pub enum AppEvent {
    Tick,
    Key(KeyEvent),
    Resize(u16, u16),
    PtyOutput,
    GlobalHotkey(u32),
    ToggleProfiler,
    ToggleWorkflows,
    ToggleDrive, // New event for Drive hotkey
    ToggleCommandPalette, // New event
    ToggleSettings, // New event for settings hotkey
    ToggleAgentMode, // New event
    ToggleFollowUp, // New event
    AgentCompleted { pane_id: Uuid, response: AgentResponse },
    CodebaseUpdate, // New event for codebase status update
    ShellExit,
    Error(String), // New event for handling errors from async tasks
}

/// An asynchronous event handler.
pub struct EventHandler {
    pub tx: mpsc::UnboundedSender<AppEvent>,
    pub rx: mpsc::UnboundedReceiver<AppEvent>,
}

impl EventHandler {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        Self { tx, rx }
    }

    /// Spawns a task to listen for terminal events (keyboard, resize)
    /// and forward them to the main event channel.
    pub fn spawn_terminal_event_handler(&self) {
        let tx = self.tx.clone();
        tokio::spawn(async move {
            let mut reader = crossterm::event::EventStream::new();
            loop {
                // Poll for a terminal event for a short duration.
                let delay = tokio::time::sleep(Duration::from_millis(16));
                tokio::select! {
                    _ = delay => {
                        if let Err(_) = tx.send(AppEvent::Tick) { break; }
                    }
                    maybe_event = reader.next() => {
                        match maybe_event {
                            Some(Ok(evt)) => {
                                let app_event = match evt {
                                    CrosstermEvent::Key(key) => AppEvent::Key(key),
                                    CrosstermEvent::Resize(w, h) => AppEvent::Resize(w, h),
                                    _ => continue, // Ignore other events
                                };
                                if tx.send(app_event).is_err() { break; }
                            }
                            Some(Err(_)) => {
                                let _ = tx.send(AppEvent::Error("Terminal event error".to_string()));
                                break;
                            }
                            None => break,
                        }
                    }
                }
            }
        });
    }
}