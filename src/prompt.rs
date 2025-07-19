// src/prompt.rs

extern crate google_ai_rs;

use std::{env, error::Error, path::PathBuf};
use dotenvy::dotenv;
use home::home_dir;
use walkdir::WalkDir;
use google_ai_rs::{Auth, Client};

/// If `input` begins with `cd `, scans $HOME for matches and asks Gemini for the best completion.
/// Otherwise returns `Ok("")`.
pub async fn get_suggestion(input: &str) -> Result<String, Box<dyn Error>> {
    dotenv().ok();
    if !input.starts_with("cd ") {
        return Ok(String::new());
    }
    let partial = input[3..].trim();
    if partial.is_empty() {
        return Ok(String::new());
    }

    let home_dir: PathBuf = home_dir().ok_or("Cannot locate home directory")?;
    let mut options = Vec::new();
    for entry in WalkDir::new(&home_dir).into_iter().filter_map(Result::ok) {
        if entry.file_type().is_dir() {
            if let Some(p) = entry.path().to_str() {
                if p.contains(partial) {
                    options.push(p.to_string());
                }
            }
        }
    }
    if options.is_empty() {
        return Ok(String::new());
    }

    let key = env::var("API_KEY").or_else(|_| env::var("GEMINI_API_KEY"))?;
    let client = Client::new(Auth::ApiKey(key)).await?;
    let prompt = format!(
        "You are a shell completion assistant.\n\
         Current dir: {}\n\
         Incomplete: {}\n\
         Candidates:\n- {}",
        env::current_dir()?.display(),
        input,
        options.join("\n- ")
    );

    let response = client
        .generative_model("gemini-2.5-flash")
        .generate_content(prompt)
        .await?;
    Ok(response.text())
}
