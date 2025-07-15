pub struct PromptTemplate {
    pub name: String,
    pub system: String,
    pub user: String,
}

impl PromptTemplate {
    pub fn render(&self, context: &str) -> String {
        self.system.replace("{{context}}", context) + "\n" + &self.user
    }
}