use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ShellCommand {
    pub command: String,
    pub args: Vec<String>,
    pub shell: String,
}

pub struct ShellTranslator {
    rules: HashMap<String, fn(&ShellCommand) -> ShellCommand>,
}

impl ShellTranslator {
    pub fn new() -> Self {
        let mut rules = HashMap::new();
        rules.insert("bash_to_fish".into(), Self::bash_to_fish);
        rules.insert("zsh_to_bash".into(), Self::zsh_to_bash);
        rules.insert("powershell_to_bash".into(), Self::powershell_to_bash);
        rules.insert("bash_to_powershell".into(), Self::bash_to_powershell);
        Self { rules }
    }

    pub fn translate(&self, cmd: &ShellCommand, target_shell: &str) -> ShellCommand {
        let key = format!("{}_to_{}", cmd.shell, target_shell);
        if let Some(rule) = self.rules.get(&key) {
            rule(cmd)
        } else {
            cmd.clone()
        }
    }

    fn bash_to_fish(cmd: &ShellCommand) -> ShellCommand {
        let mut translated = cmd.clone();
        if cmd.command == "export" && !cmd.args.is_empty() {
            let parts: Vec<&str> = cmd.args[0].split('=').collect();
            if parts.len() == 2 {
                translated.command = "set".into();
                translated.args = vec!["-x".into(), parts[0].into(), parts[1].into()];
                translated.shell = "fish".into();
            }
        }
        translated
    }

    fn zsh_to_bash(cmd: &ShellCommand) -> ShellCommand {
        let mut translated = cmd.clone();
        translated.shell = "bash".into();
        translated
    }

    fn bash_to_powershell(cmd: &ShellCommand) -> ShellCommand {
        let mut translated = cmd.clone();
        if cmd.command == "export" && !cmd.args.is_empty() {
            let parts: Vec<&str> = cmd.args[0].split('=').collect();
            if parts.len() == 2 {
                translated.command = "$env:".to_owned() + parts[0];
                translated.args = vec![parts[1].into()];
                translated.shell = "powershell".into();
            }
        }
        translated
    }

    fn powershell_to_bash(cmd: &ShellCommand) -> ShellCommand {
        let mut translated = cmd.clone();
        if cmd.command.starts_with("$env:") && !cmd.args.is_empty() {
            let key = cmd.command.trim_start_matches("$env:");
            translated.command = "export".into();
            translated.args = vec![format!("{}={}", key, cmd.args[0])];
            translated.shell = "bash".into();
        }
        translated
    }
}