use ratatui::style::{Color, Style};
use vte::ansi::{NamedColor, Rgb};

/// Represents a single cell on the terminal grid.
#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    pub char: char,
    pub style: Style,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            char: ' ',
            style: Style::default(),
        }
    }
}

/// A grid representing the state of the terminal screen.
#[derive(Debug, Clone)]
pub struct Grid {
    pub width: u16,
    pub height: u16,
    pub cells: Vec<Cell>,
    pub cursor_x: u16,
    pub cursor_y: u16,
    pub show_cursor: bool,
}

impl Grid {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            cells: vec![Cell::default(); (width * height) as usize],
            cursor_x: 0,
            cursor_y: 0,
            show_cursor: true,
        }
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.cells.resize((width * height) as usize, Cell::default());
    }

    pub fn get_mut(&mut self, x: u16, y: u16) -> Option<&mut Cell> {
        if x < self.width && y < self.height {
            self.cells.get_mut((y * self.width + x) as usize)
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.cells.iter_mut().for_each(|c| *c = Cell::default());
    }

    /// Converts a VTE color to a Ratatui color.
    pub fn vte_color_to_ratatui(color: vte::ansi::Color) -> Color {
        match color {
            vte::ansi::Color::Named(c) => match c {
                NamedColor::Black => Color::Black,
                NamedColor::Red => Color::Red,
                NamedColor::Green => Color::Green,
                NamedColor::Yellow => Color::Yellow,
                NamedColor::Blue => Color::Blue,
                NamedColor::Magenta => Color::Magenta,
                NamedColor::Cyan => Color::Cyan,
                NamedColor::White => Color::White,
                NamedColor::BrightBlack => Color::Gray,
                NamedColor::BrightRed => Color::LightRed,
                NamedColor::BrightGreen => Color::LightGreen,
                NamedColor::BrightYellow => Color::LightYellow,
                NamedColor::BrightBlue => Color::LightBlue,
                NamedColor::BrightMagenta => Color::LightMagenta,
                NamedColor::BrightCyan => Color::LightCyan,
                NamedColor::BrightWhite => Color::White,
                NamedColor::DimBlack => Color::DarkGray,
                NamedColor::DimRed => Color::Red,
                NamedColor::DimGreen => Color::Green,
                NamedColor::DimYellow => Color::Yellow,
                NamedColor::DimBlue => Color::Blue,
                NamedColor::DimMagenta => Color::Magenta,
                NamedColor::DimCyan => Color::Cyan,
                NamedColor::DimWhite => Color::Gray,
                NamedColor::Foreground => Color::Reset,
                NamedColor::Background => Color::Reset,
            },
            vte::ansi::Color::Spec(Rgb { r, g, b }) => Color::Rgb(r, g, b),
            vte::ansi::Color::Indexed(i) => Color::Indexed(i),
        }
    }
} 