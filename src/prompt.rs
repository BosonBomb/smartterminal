extern crate google_ai_rs;
use std::{env, error::Error, fs, path::PathBuf};
use walkdir::WalkDir;
use dotenvy::dotenv;

// Import the type-safe SDK
use google_ai_rs::{Client, generative::GenerativeModel};

/// Returns a shell completion suggestion by querying Google Gemini.
/// Propagates errors so the caller can handle them.
pub async fn get_suggestion(input: &str) -> Result<String, Box<dyn Error>> {
    dotenv().ok(); // Load .env variables if present

    // Only complete `cd ` commands
    if !input.starts_with("cd ") {
        return Ok(String::new());
    }

    let partial = input[3..].trim();
    if partial.is_empty() {
        return Ok(String::new());
    }

    // Collect matching directory paths under $HOME
    let home_dir: PathBuf = home::home_dir().ok_or("Could not determine home directory")?;
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

    // Load API key from environment
    let api_key = env::var("API_KEY").or_else(|_| env::var("GEMINI_API_KEY"))?;
    let client = Client::new(api_key).await?;

    // Build prompt for Gemini
    let prompt = format!(
        "You are a shell completion assistant. Current dir: {}\nIncomplete: {}\nCandidates:\n- {}",
        env::current_dir()?.display(),
        input,
        options.join("\n- ")
    );

    // Call Gemini model
    let response = client
        .generative_model("gemini-2.5-flash")
        .generate_content(&prompt)
        .await?;

    Ok(response.text)
}