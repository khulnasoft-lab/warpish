use ratatui::prelude::*;

pub fn draw_layout<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(f.size());

    let sidebar = Block::default().title("Sidebar").borders(Borders::ALL);
    let main = Block::default().title("Terminal").borders(Borders::ALL);

    f.render_widget(sidebar, chunks[0]);
    f.render_widget(main, chunks[1]);
}