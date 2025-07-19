// src/terminal.rs

use std::error::Error;
use std::io::{self, Write};
use crate::{command::execute_command, prompt::get_suggestion};

/// Runs the interactive loop:
/// 1. Reads a line
/// 2. Fetches a `cd` suggestion (if applicable) and prints it
/// 3. Invokes the suggestion as a real command
pub async fn run() -> Result<(), Box<dyn Error>> {
    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim_end();

        // Ask Gemini for a completion
        let suggestion = get_suggestion(&input).await?;
        if !suggestion.is_empty() {
            println!("Suggestion: {}", suggestion);

            // Optionally execute it
            let parts: Vec<&str> = suggestion.split_whitespace().collect();
            if let Some((prog, args)) = parts.split_first() {
                let status = execute_command(prog, args)?;
                println!("Exited with: {}", status);
            }
        }
    }
}
