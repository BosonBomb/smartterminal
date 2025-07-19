// src/command.rs

use std::error::Error;
use std::process::{Command, ExitStatus};

/// Spawns `program` with `args`, waits for it to finish, and returns its exit status.
pub fn execute_command(program: &str, args: &[&str]) -> Result<ExitStatus, Box<dyn Error>> {
    // `child.wait()` requires &mut self, so we must declare `mut child`
    let mut child = Command::new(program)
        .args(args)
        .spawn()?;
    let status = child.wait()?;
    Ok(status)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_echo() {
        let status = execute_command("echo", &["hello"]).unwrap();
        assert!(status.success());
    }
}
