pub fn detect_shell(cmd: &str) -> Option<&'static str> {
    if cmd.contains("function") || cmd.contains("setopt") {
        Some("zsh")
    } else if cmd.contains("export") || cmd.contains("alias") {
        Some("bash")
    } else if cmd.contains("set -x") || cmd.contains("function fish") {
        Some("fish")
    } else {
        None
    }
}