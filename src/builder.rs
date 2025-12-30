use std::collections::HashMap;
use std::process::{Command, exit};
use crate::parser::Target;
use crate::utils::needs_rebuild;

pub fn build_target(target_name: &str, targets: &HashMap<String, Target>) {
    if let Some(target) = targets.get(target_name) {
        // Build dependencies first
        for dep in &target.dependencies {
            build_target(dep, targets); // Recursively build dependencies
        }

        if needs_rebuild(target_name, &target.dependencies) {
            println!("Building target: {}", target_name);
            let command = &target.command;

            // Debugging output to verify the command before running it
            println!("Running command: {}", command);

            let status = if cfg!(windows) {
                // Use `cmd.exe` on Windows
                let output = Command::new("cmd")
                    .args(&["/C", command])
                    .output()
                    .expect("Failed to execute command");

                // Debugging output for capturing the output
                if !output.status.success() {
                    eprintln!("Error executing command on Windows: {}", String::from_utf8_lossy(&output.stderr));
                } else {
                    println!("Command output: {}", String::from_utf8_lossy(&output.stdout));
                }
                output.status
            } else {
                // Use `sh` on Unix-like systems
                let output = Command::new("sh")
                    .arg("-c")
                    .arg(command)
                    .output()  // Capture output and error
                    .expect("Failed to execute command");

                // Debugging output for capturing the output
                if !output.status.success() {
                    eprintln!("Error executing command on Unix-like system: {}", String::from_utf8_lossy(&output.stderr));
                } else {
                    println!("Command output: {}", String::from_utf8_lossy(&output.stdout));
                }
                output.status
            };

            if !status.success() {
                eprintln!("Error during build of target: {}", target_name);
                exit(1);
            }
        }
    }
}
