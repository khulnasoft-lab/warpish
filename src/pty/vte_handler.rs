use std::sync::{Arc, Mutex};
use vte::{ansi, Perform, Parser};

// Simplified grid implementation
#[derive(Debug)]
pub struct SimpleGrid {
    pub rows: u16,
    pub cols: u16,
    pub content: Vec<Vec<char>>,
}

impl SimpleGrid {
    pub fn new(rows: u16, cols: u16) -> Self {
        Self {
            rows,
            cols,
            content: vec![vec![' '; cols as usize]; rows as usize],
        }
    }

    pub fn resize(&mut self, rows: u16, cols: u16) {
        self.rows = rows;
        self.cols = cols;
        self.content = vec![vec![' '; cols as usize]; rows as usize];
    }

    pub fn clear(&mut self) {
        self.content = vec![vec![' '; self.cols as usize]; self.rows as usize];
    }

    pub fn rows_iter(&self) -> impl Iterator<Item = &Vec<char>> {
        self.content.iter()
    }
}

// Implement the Perform trait for our simple grid handler
impl Perform for SimpleGrid {
    fn print(&mut self, c: char) {
        // Simple implementation - just store the character
        // In a real implementation, you'd track cursor position
        if let Some(row) = self.content.get_mut(0) {
            if let Some(cell) = row.get_mut(0) {
                *cell = c;
            }
        }
    }

    fn execute(&mut self, _byte: u8) {
        // Handle control characters
    }

    fn csi_dispatch(
        &mut self,
        _params: &vte::Params,
        _intermediates: &[u8],
        _ignore: bool,
        _action: char,
    ) {
        // Handle CSI sequences
    }

    fn esc_dispatch(&mut self, _intermediates: &[u8], _ignore: bool, _byte: u8) {
        // Handle escape sequences
    }
}

// The handler now wraps our SimpleGrid
#[derive(Debug)]
pub struct VteHandler {
    grid: Arc<Mutex<SimpleGrid>>,
}

impl VteHandler {
    pub fn new(rows: u16, cols: u16) -> Self {
        Self {
            grid: Arc::new(Mutex::new(SimpleGrid::new(rows, cols))),
        }
    }

    pub fn get_grid(&self) -> std::sync::MutexGuard<'_, SimpleGrid> {
        self.grid.lock().unwrap()
    }

    pub fn process_input(&mut self, data: &[u8], parser: &mut Parser) {
        let mut grid_lock = self.grid.lock().unwrap();
        parser.advance(&mut *grid_lock, data);
    }

    pub fn resize(&mut self, rows: u16, cols: u16) {
        let mut grid_lock = self.grid.lock().unwrap();
        grid_lock.resize(rows, cols);
    }

    pub fn clear_screen(&mut self) {
        let mut grid_lock = self.grid.lock().unwrap();
        grid_lock.clear();
    }

    pub fn to_string(&self) -> String {
        let grid_lock = self.grid.lock().unwrap();
        let mut s = String::new();
        for row in grid_lock.rows_iter() {
            let row_text: String = row.iter().collect();
            s.push_str(&row_text);
            s.push('\n');
        }
        s
    }
}

// FIX: Make this struct public
pub struct VteState {
    pub parser: Parser,
    handler: VteHandler,
}

impl VteState {
    pub fn new(rows: u16, cols: u16) -> Self {
        Self {
            parser: Parser::new(),
            handler: VteHandler::new(rows, cols),
        }
    }

    pub fn get_grid(&self) -> std::sync::MutexGuard<'_, SimpleGrid> {
        self.handler.get_grid()
    }
}
