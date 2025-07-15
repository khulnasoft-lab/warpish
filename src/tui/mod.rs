use crate::{error::AppResult, ui};
use crossterm::{cursor, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}};
use ratatui::prelude::{CrosstermBackend, Terminal};
use std::{io, panic};

/// Represents the Terminal UI.
/// It's responsible for setting up the terminal, drawing the UI,
/// and restoring the terminal state on exit.
pub struct Tui {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl Tui {
    pub fn new() -> AppResult<Self> {
        let terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
        Ok(Self { terminal })
    }

    pub fn enter(&mut self) -> AppResult<()> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(io::stdout(), EnterAlternateScreen, cursor::Show)?;
        Self::init_panic_hook();
        Ok(())
    }

    pub fn exit(&mut self) -> AppResult<()> {
        terminal::disable_raw_mode()?;
        crossterm::execute!(io::stdout(), LeaveAlternateScreen, cursor::Show)?;
        Ok(())
    }

    pub fn draw(&mut self, app: &mut crate::app::App) -> AppResult<()> {
        self.terminal.draw(|frame| ui::render(frame, app))?;
        Ok(())
    }
    
    fn init_panic_hook() {
        let original_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            let mut stdout = io::stdout();
            let _ = terminal::disable_raw_mode();
            let _ = crossterm::execute!(stdout, LeaveAlternateScreen, cursor::Show);
            original_hook(panic_info);
        }));
    }
}