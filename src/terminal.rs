use crate::command::handle_command;
use crate::prompt::get_suggestion;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::io::{self, stdout};

pub async fn run() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();

    let mut input = String::new();
    let mut suggestion = String::new();

    loop {
        let current_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("/"));
        let display_dir = if let Some(home_dir) = home::home_dir() {
            if let Ok(stripped) = current_dir.strip_prefix(&home_dir) {
                format!("~/{}>", stripped.display())
            } else {
                format!("{}>", current_dir.display())
            }
        } else {
            format!("{}>", current_dir.display())
        };

        execute!(
            stdout,
            Clear(ClearType::All),
            Print(&display_dir),
            Print(&input),
            SetForegroundColor(Color::Grey),
            Print(&suggestion),
            ResetColor
        )?;

        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Char(c) => {
                    input.push(c);
                    suggestion = get_suggestion(&input).await;
                }
                KeyCode::Backspace => {
                    input.pop();
                    suggestion = get_suggestion(&input).await;
                }
                KeyCode::Tab => {
                    input.push_str(&suggestion);
                    suggestion.clear();
                }
                KeyCode::Enter => {
                    if input.trim() == "exit" {
                        break;
                    }
                    handle_command(&input);
                    input.clear();
                    suggestion.clear();
                }
                KeyCode::Esc => {
                    break;
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    Ok(())
}
