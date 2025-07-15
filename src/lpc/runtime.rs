use super::{detector::detect_shell, translator::{ShellTranslator, ShellCommand}, validator::validate};

pub fn process_command(input: &str, current_shell: &str, target_shell: &str) -> Option<String> {
    let detected = detect_shell(input).unwrap_or(current_shell);
    if !validate(input, detected) {
        eprintln!("Invalid syntax for shell: {}", detected);
        return None;
    }

    let args: Vec<String> = input.split_whitespace().skip(1).map(|s| s.to_string()).collect();
    let command = input.split_whitespace().next().unwrap_or("").to_string();

    let shell_cmd = ShellCommand {
        command,
        args,
        shell: detected.into(),
    };

    let translator = ShellTranslator::new();
    let translated = translator.translate(&shell_cmd, target_shell);

    Some(format!("{} {}", translated.command, translated.args.join(" ")))
}