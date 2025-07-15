pub fn validate_bash(cmd: &str) -> bool {
    !cmd.contains(";;")
}

pub fn validate_fish(cmd: &str) -> bool {
    !cmd.contains("function()")
}

pub fn validate(cmd: &str, shell: &str) -> bool {
    match shell {
        "bash" => validate_bash(cmd),
        "fish" => validate_fish(cmd),
        _ => true,
    }
}