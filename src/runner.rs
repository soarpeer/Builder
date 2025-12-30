use std::process::{Command, exit};
use std::collections::HashMap;

pub fn run(targets: &HashMap<String, crate::parser::Target>) {
    if let Some(target) = targets.get("run") {
        let status = Command::new("sh")
            .arg("-c")
            .arg(&target.command)
            .status()
            .expect("Failed to execute command");

        if !status.success() {
            eprintln!("Error during run.");
            exit(1);
        }
    }
}
