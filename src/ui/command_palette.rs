use ratatui::{prelude::*, widgets::*};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

pub struct CommandPalette {
    pub input: String,
    pub matches: Vec<(String, i64)>,
    pub all_commands: Vec<String>,
}

impl CommandPalette {
    pub fn new(commands: Vec<String>) -> Self {
        Self {
            input: String::new(),
            matches: vec![],
            all_commands: commands,
        }
    }

    pub fn update_matches(&mut self) {
        let matcher = SkimMatcherV2::default();
        self.matches = self
            .all_commands
            .iter()
            .filter_map(|cmd| matcher.fuzzy_match(cmd, &self.input).map(|score| (cmd.clone(), score)))
            .collect();
        self.matches.sort_by(|a, b| b.1.cmp(&a.1));
    }

    pub fn render<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let items: Vec<ListItem> = self.matches.iter()
            .map(|(text, _)| ListItem::new(text.clone()))
            .collect();

        let block = Block::default().borders(Borders::ALL).title("Command Palette");
        let list = List::new(items).block(block);

        f.render_widget(list, area);
    }
}