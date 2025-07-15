//! Terminal UI Demo
//! 
//! This binary demonstrates the Warp-inspired Blocks terminal UI functionality.

use warpish_terminal_v2::ui::terminal_ui::TerminalUI;
use std::io;

fn main() -> Result<(), io::Error> {
    // Create and run the terminal UI
    let mut ui = TerminalUI::new()?;
    
    println!("Starting Warpish Terminal UI Demo...");
    println!("Press 'h' for help once the UI starts.");
    
    // Small delay to let user see the message
    std::thread::sleep(std::time::Duration::from_millis(1000));
    
    ui.run()?;
    
    println!("Terminal UI Demo finished. Thanks for trying it out!");
    Ok(())
}
