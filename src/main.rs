// src/main.rs

mod prompt;
mod terminal;
mod command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    terminal::run().await?;
    Ok(())
}
