use google_generative_ai_rs::v1::gemini::{Content, GenerativeModel, Part, Role};
use std::env;
use walkdir::WalkDir;

pub async fn get_suggestion(input: &str) -> String {
    if !input.starts_with("cd ") {
        return String::new();
    }

    let partial_path = &input[3..];
    if partial_path.is_empty() {
        return String::new();
    }

    let home_dir = match home::home_dir() {
        Some(dir) => dir,
        None => return String::new(),
    };

    let mut candidates = Vec::new();
    for entry in WalkDir::new(&home_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_dir() {
            if let Some(path_str) = entry.path().to_str() {
                if path_str.contains(partial_path) {
                    candidates.push(path_str.to_string());
                }
            }
        }
    }

    if candidates.is_empty() {
        return String::new();
    }

    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not set");
    let generative_model =
        GenerativeModel::new("gemini-1.5-flash", api_key).expect("Failed to create generative model");

    let prompt = format!(
        "You are an expert shell command assistant. Your task is to complete the user's command based on the context provided. Provide only the single, most likely shell command as your answer.

---
**Context:**
1.  **User's Current Directory:** {}
2.  **User's Incomplete Command:** {}
3.  **Search Results:** My script found the following possible full paths for \"{}\":
    - {}
---

Based on this context, what is the most likely command the user wants to execute?",
        env::current_dir().unwrap().display(),
        input,
        partial_path,
        candidates.join("\n    - ")
    );

    let content = Content {
        role: Role::User,
        parts: vec![Part {
            text: Some(prompt),
            ..Default::default()
        }],
    };

    match generative_model.generate_content(vec![content]).await {
        Ok(response) => {
            if let Some(candidate) = response.candidates.first() {
                if let Some(part) = candidate.content.parts.first() {
                    if let Some(text) = &part.text {
                        return text.trim_start_matches(input).to_string();
                    }
                }
            }
            String::new()
        }
        Err(_) => String::new(),
    }
}
