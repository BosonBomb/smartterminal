mod command;
mod prompt;
mod terminal;

use dotenvy::dotenv;
use std::{env, io};

#[tokio::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    if let Some(home_dir) = home::home_dir() {
        env::set_current_dir(&home_dir)?;
    }

    terminal::run().await
}
