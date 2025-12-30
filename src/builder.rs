use std::collections::HashMap;
use std::process::{Command, exit};
use std::fs;
use crate::parser::Target;
use crate::utils::needs_rebuild;

pub fn build_target(target_name: &str, targets: &HashMap<String, Target>) {
    if let Some(target) = targets.get(target_name) {
        for dep in &target.dependencies {
            build_target(dep, targets); // Recursively build dependencies
        }

        if needs_rebuild(target_name, &target.dependencies) {
            println!("Building target: {}", target_name);
            let status = Command::new("sh")
                .arg("-c")
                .arg(&target.command)
                .status()
                .expect("Failed to execute command");

            if !status.success() {
                eprintln!("Error during build of target: {}", target_name);
                exit(1);
            }
        }
    }
}
