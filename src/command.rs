use std::env;
use std::path::Path;
use std::process::Command;

pub fn handle_command(input: &str) {
    let mut parts = input.trim().split_whitespace();
    let command = parts.next().unwrap_or("");
    let args: Vec<&str> = parts.collect();

    match command {
        "cd" => {
            let new_dir = args.get(0).map_or_else(
                || home::home_dir().unwrap_or_else(|| Path::new("/").to_path_buf()),
                |x| Path::new(x).to_path_buf(),
            );
            if let Err(e) = env::set_current_dir(&new_dir) {
                eprintln!("cd: {}", e);
            }
        }
        "" => (),
        _ => {
            let mut child = Command::new(command)
                .args(args)
                .spawn();

            match child {
                Ok(mut child) => {
                    if let Err(e) = child.wait() {
                        eprintln!("Error waiting for command: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("{}: {}", command, e);
                }
            }
        }
    }
}