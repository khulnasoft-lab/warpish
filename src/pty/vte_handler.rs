use std::sync::{Arc, Mutex};
use vte::{Parser, Perform, ansi};

// Define our own Grid and GridCoords
pub struct Grid {
    // Add necessary fields
    pub rows: usize,
    pub cols: usize,
}

pub struct GridCoords {
    pub x: usize,
    pub y: usize,
}
use ratatui::style::{Color as RatatuiColor, Modifier, Style};

/// A VTE event-handler that updates a grid.
#[derive(Debug)]
struct VteActor {
    grid: Arc<Mutex<Grid>>,
}

impl VteActor {
    fn new(grid: Arc<Mutex<Grid>>) -> Self {
        VteActor { grid }
    }
}

/// Implement the vte::Perform trait to handle escape sequences.
/// The parser will call these methods as it processes the byte stream.
impl Perform for VteActor {
    fn print(&mut self, c: char) {
        let mut grid = self.grid.lock().unwrap();
        grid.input(c);
    }

    fn execute(&mut self, byte: u8) {
        let mut grid = self.grid.lock().unwrap();
        grid.input(byte as char);
    }

    fn csi_dispatch(
        &mut self,
        params: &vte::Params,
        intermediates: &[u8],
        ignore: bool,
        action: char,
    ) {
        let mut grid = self.grid.lock().unwrap();
        grid.csi_dispatch(params, intermediates, ignore, action);
    }

    fn esc_dispatch(&mut self, intermediates: &[u8], ignore: bool, byte: u8) {
        let mut grid = self.grid.lock().unwrap();
        grid.esc_dispatch(intermediates, ignore, byte);
    }
}

/// The main struct that holds the terminal state.
pub struct VteState {
    parser: Parser,
    grid: Arc<Mutex<Grid>>,
}

impl VteState {
    pub fn new(cols: u16, rows: u16) -> Self {
        let grid = Arc::new(Mutex::new(Grid::new(
            rows as usize,
            cols as usize,
            0, // No scrollback buffer in the grid itself
        )));
        let performer = VteActor::new(grid.clone());
        let parser = Parser::new();

        VteState { parser, grid }
    }

    /// Process incoming bytes from the PTY.
    pub fn process(&mut self, data: &[u8]) {
        let mut performer = VteActor::new(self.grid.clone());
        for byte in data {
            self.parser.advance(&mut performer, *byte);
        }
    }

    /// Resize the terminal grid.
    pub fn resize(&mut self, cols: u16, rows: u16) {
        let mut grid = self.grid.lock().unwrap();
        grid.resize(rows as usize, cols as usize);
    }

    /// Provides locked access to the grid for rendering.
    pub fn get_grid(&self) -> std::sync::MutexGuard<'_, Grid> {
        self.grid.lock().unwrap()
    }

    /// Clears the entire grid, including the scrollback buffer.
    pub fn clear_all(&mut self) {
        let mut grid = self.grid.lock().unwrap();
        grid.clear_history();
        grid.clear_screen(ansi::ClearMode::All);
        grid.goto(GridCoords { row: 0, col: 0 });
    }

    /// A simple heuristic to parse the grid content into blocks.
    /// A block is a set of contiguous non-empty lines.
    pub fn get_blocks(&self) -> Vec<String> {
        let grid = self.grid.lock().unwrap();
        let mut blocks = Vec::new();
        let mut current_block = String::new();
        for i in 0..grid.height() {
            let row_text: String = grid.row(i).iter().map(|cell| cell.c).collect();
            if row_text.trim().is_empty() {
                if !current_block.is_empty() {
                    blocks.push(current_block.trim().to_string());
                    current_block.clear();
                }
            } else {
                current_block.push_str(&row_text);
                current_block.push_str("\n");
            }
        }
        if !current_block.is_empty() {
            blocks.push(current_block.trim().to_string());
        }
        blocks
    }
}

/// Helper function to convert VTE colors to Ratatui colors.
pub fn vte_color_to_ratatui(color: ansi::Color) -> RatatuiColor {
    match color {
// Handle default color
        _ => RatatuiColor::Reset,
        ansi::Color::Named(c) => match c {
            ansi::NamedColor::Black => RatatuiColor::Black,
            ansi::NamedColor::Red => RatatuiColor::Red,
            ansi::NamedColor::Green => RatatuiColor::Green,
            ansi::NamedColor::Yellow => RatatuiColor::Yellow,
            ansi::NamedColor::Blue => RatatuiColor::Blue,
            ansi::NamedColor::Magenta => RatatuiColor::Magenta,
            ansi::NamedColor::Cyan => RatatuiColor::Cyan,
            ansi::NamedColor::White => RatatuiColor::White,
            ansi::NamedColor::BrightBlack => RatatuiColor::Gray,
            ansi::NamedColor::BrightRed => RatatuiColor::LightRed,
            ansi::NamedColor::BrightGreen => RatatuiColor::LightGreen,
            ansi::NamedColor::BrightYellow => RatatuiColor::LightYellow,
            ansi::NamedColor::BrightBlue => RatatuiColor::LightBlue,
            ansi::NamedColor::BrightMagenta => RatatuiColor::LightMagenta,
            ansi::NamedColor::BrightCyan => RatatuiColor::LightCyan,
            ansi::NamedColor::BrightWhite => RatatuiColor::White,
            ansi::NamedColor::Foreground => RatatuiColor::Reset,
            ansi::NamedColor::Background => RatatuiColor::Reset,
            ansi::NamedColor::DimBlack => RatatuiColor::DarkGray,
            ansi::NamedColor::DimRed => RatatuiColor::Red,
            ansi::NamedColor::DimGreen => RatatuiColor::Green,
            ansi::NamedColor::DimYellow => RatatuiColor::Yellow,
            ansi::NamedColor::DimBlue => RatatuiColor::Blue,
            ansi::NamedColor::DimMagenta => RatatuiColor::Magenta,
            ansi::NamedColor::DimCyan => RatatuiColor::Cyan,
            ansi::NamedColor::DimWhite => RatatuiColor::Gray,
        },
        ansi::Color::Spec(rgb) => RatatuiColor::Rgb(rgb.r, rgb.g, rgb.b),
        ansi::Color::Indexed(idx) => RatatuiColor::Indexed(idx),
    }
}

/// Helper to convert VTE cell flags to Ratatui Style modifiers.
pub fn vte_flags_to_ratatui_style(flags: u32) -> Style {
    let mut style = Style::default();
// Replace vte::Flags with our own flags implementation
if flags & 1 != 0 { // BOLD flag
        style = style.add_modifier(Modifier::BOLD);
    }
if flags & 2 != 0 { // ITALIC flag
        style = style.add_modifier(Modifier::ITALIC);
    }
if flags & 4 != 0 { // UNDERLINE flag
        style = style.add_modifier(Modifier::UNDERLINED);
    }
if flags & 8 != 0 { // INVERSE flag
        style = style.add_modifier(Modifier::REVERSED);
    }
if flags & 16 != 0 { // STRIKETHROUGH flag
        style = style.add_modifier(Modifier::CROSSED_OUT);
    }
    style
}