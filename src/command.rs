use std::env;
use std::path::Path;

pub fn handle_command(input: &str) {
    let mut parts = input.trim().split_whitespace();
    let command = parts.next().unwrap_or("");
    let args = parts;

    match command {
        "cd" => {
            let new_dir = args.peekable().peek().map_or_else(
                || home::home_dir().unwrap_or_else(|| Path::new("/").to_path_buf()),
                |x| Path::new(x).to_path_buf(),
            );
            if let Err(e) = env::set_current_dir(&new_dir) {
                eprintln!("cd: {}", e);
            }
        }
        "" => (),
        _ => {
            eprintln!("{}: command not found", command);
        }
    }
}
