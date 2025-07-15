use crate::agent::client::AgentResponse;
use crate::agent::model::ModelId;
use crate::event::AppEvent;
use crate::pty::vte_handler::VteState;
use portable_pty::{CommandBuilder, NativePtySystem, PtyPair, PtySize, PtySystem};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use uuid::Uuid;
use winit::event_loop::EventLoopProxy;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentStatus {
    InProgress,
    WaitingForInput,
    Done,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentState {
    pub id: Uuid,
    pub status: AgentStatus,
    pub task_summary: String,
    pub conversation: Vec<(String, AgentResponse)>,
    pub current_input: String,
    pub is_follow_up: bool,
    pub attached_context_blocks: Vec<usize>, // Indices into the VTE handler's block list
    pub model_used: ModelId, // Track the model for this conversation
}

// A new struct to represent one atomic command/output unit.
#[derive(Debug, Clone)]
pub struct Block {
    pub id: Uuid,
    pub command: String,
    pub output: String,
}

pub struct Pane {
    pub id: Uuid,
    // The live VTE session for the current command
    pub current_vte: Arc<Mutex<VteState>>,
    // The history of completed command/output blocks
    pub history: Vec<Block>,
    // The command that is currently running or was just entered
    pub active_command: String,
    pub pty_writer: Box<dyn Write + Send>,
    pty_pair: PtyPair,
    pub agent_state: Option<AgentState>,
}

impl Pane {
    pub fn new(
        cols: u16,
        rows: u16,
        shell_str: &str,
        event_proxy: EventLoopProxy<AppEvent>,
    ) -> Self {
        let pty_system = NativePtySystem::default();
        let pty_pair = pty_system
            .openpty(PtySize {
                rows,
                cols,
                ..Default::default()
            })
            .expect("Failed to open PTY");

        let mut cmd = CommandBuilder::new(shell_str);
        cmd.env("TERM_PROGRAM", "WarpishTerminal");
        cmd.cwd(std::env::current_dir().unwrap());

        let _child = pty_pair
            .slave
            .spawn_command(cmd)
            .expect("Failed to spawn shell");

        let pty_writer = pty_pair.master.take_writer().unwrap();
        let pty_reader = pty_pair.master.try_clone_reader().unwrap();

        let current_vte = Arc::new(Mutex::new(VteState::new(cols, rows)));
        let vte_clone = current_vte.clone();

        // The reader thread now only writes to the current VTE
        thread::spawn(move || {
            let mut reader = pty_reader;
            let mut buffer = [0u8; 8192];
            loop {
                match reader.read(&mut buffer) {
                    Ok(0) | Err(_) => break,
                    Ok(n) => {
                        vte_clone.lock().unwrap().process(&buffer[..n]);
                        event_proxy.send_event(AppEvent::PtyOutput).ok();
                    }
                }
            }
        });

        Pane {
            id: Uuid::new_v4(),
            current_vte,
            history: Vec::new(),
            active_command: String::new(),
            pty_writer,
            pty_pair,
            agent_state: None,
        }
    }

    /// "Seals" the current VTE state into a historical block.
    pub fn new_block(&mut self) {
        let mut vte = self.current_vte.lock().unwrap();
        let output = vte.get_grid().as_ref().to_string();
        vte.clear_all(); // Clear the VTE for the next command
        let block = Block {
            id: Uuid::new_v4(),
            command: self.active_command.clone(),
            output,
        };
        self.history.push(block);
    }

    pub fn resize(&self, cols: u16, rows: u16) {
        self.current_vte.lock().unwrap().resize(cols, rows);
        self.pty_pair
            .master
            .resize(PtySize {
                rows,
                cols,
                ..Default::default()
            })
            .ok();
    }

    pub fn enter_agent_mode(&mut self, initial_query: String, model: ModelId) {
        if self.agent_state.is_none() {
            self.agent_state = Some(AgentState {
                id: Uuid::new_v4(),
                status: AgentStatus::InProgress,
                task_summary: initial_query.clone(),
                conversation: vec![],
                current_input: initial_query,
                is_follow_up: false,
                attached_context_blocks: vec![],
                model_used: model, // Store the model
            });
        }
    }
    pub fn start_new_conversation(&mut self) {
        if let Some(state) = &mut self.agent_state {
            state.conversation.clear();
            state.is_follow_up = false;
        }
    }
    pub fn exit_agent_mode(&mut self) {
        self.agent_state = None;
    }
}