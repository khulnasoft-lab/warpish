use std::io::Write;

/// Takes an action name and a PTY writer, and sends the corresponding
/// byte sequence for that action.
pub fn perform_action(
    action: &str,
    pty_writer: &mut Box<dyn Write + Send>,
) -> Result<(), std::io::Error> {
    log::info!("Performing action: {}", action);

    // This maps the abstract action names from the YAML to the raw byte codes
    // that a terminal like `bash` or `zsh` running in Emacs mode would understand.
    let bytes: &[u8] = match action {
        // For selection, we send the non-shifted cursor movement command.
        // The shell/editor inside the terminal is responsible for interpreting this as selection.
        "editor_view:select_left" => b"\x02",        // Ctrl-B (backward-char)
        "editor_view:select_right" => b"\x06",       // Ctrl-F (forward-char)
        "editor:select_to_line_end" => b"\x05",    // Ctrl-E (end-of-line)
        "editor:select_to_line_start" => b"\x01",  // Ctrl-A (beginning-of-line)
        "editor_view:select_right_by_word" => b"\x1bf", // Meta-F (forward-word)
        "editor_view:select_left_by_word" => b"\x1bb",  // Meta-B (backward-word)
        "editor_view:cut_word_right" => b"\x1bd", // Meta-D (kill-word)
        _ => {
            log::warn!("Unknown action: {}", action);
            &[]
        }
    };

    if !bytes.is_empty() {
        pty_writer.write_all(bytes)?;
        pty_writer.flush()?;
    }
    Ok(())
} 