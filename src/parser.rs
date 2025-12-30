use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
pub struct Target {
    pub dependencies: Vec<String>,
    pub command: String,
}

pub fn parse_buildfile() -> Result<HashMap<String, Target>, io::Error> {
    let mut targets = HashMap::new();
    let buildfile_path = "buildfile.txt"; 

    let file = File::open(buildfile_path)?;
    let reader = io::BufReader::new(file);

    let mut current_target: Option<String> = None;
    let mut current_command: Option<String> = None;
    let mut dependencies = Vec::new();

    for line in reader.lines() {
        let line = line?;

        // Debugging: print the line to check how it's being parsed
        println!("Parsing line: '{}'", line);

        if line.starts_with('#') || line.trim().is_empty() {
            continue; // Skip comments and empty lines
        }

        // Parse target definition (e.g., 'build: src/main.rs')
        if let Some(pos) = line.find(':') {
            if let Some(target) = current_target.take() {
                // Save the previous target
                targets.insert(target, Target {
                    dependencies,
                    command: current_command.take().unwrap_or_default(),
                });
            }

            // Start a new target
            let target_name = line[..pos].trim().to_string();
            current_target = Some(target_name);
            dependencies = line[pos + 1..].trim().split_whitespace().map(String::from).collect();

            // Reset command to None for the new target
            current_command = None;
        } else if line.starts_with('\t') {
            // Command line (indented by a tab)
            if let Some(target) = current_target.as_mut() {
                // Capture the command by stripping leading tabs
                let command = line.trim_start(); // Trim any leading spaces or tabs
                current_command = Some(command.to_string());

                // Debugging: print the command to verify
                println!("Captured command: '{}'", command);
            }
        }
    }

    // Add the last target
    if let Some(target) = current_target {
        targets.insert(target, Target {
            dependencies,
            command: current_command.unwrap_or_default(),
        });
    }

    // Debugging output to check what we parsed
    for (target_name, target) in &targets {
        println!("Target: '{}', Command: '{}'", target_name, target.command);
    }

    Ok(targets) // Return the parsed targets wrapped in Ok
}
