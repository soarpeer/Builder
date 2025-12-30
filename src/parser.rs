use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Target {
    pub dependencies: Vec<String>,
    pub command: String,
}

pub fn parse_buildfile(file_path: &str) -> io::Result<HashMap<String, Target>> {
    let mut targets = HashMap::new();
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut current_target: Option<String> = None;
    let mut current_command: Option<String> = None;
    let mut dependencies = Vec::new();

    for line in reader.lines() {
        let line = line?;

        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }

        if let Some(pos) = line.find(':') {
            if let Some(target) = current_target.take() {
                targets.insert(target, Target {
                    dependencies,
                    command: current_command.take().unwrap_or_default(),
                });
            }

            let target_name = line[..pos].trim().to_string();
            current_target = Some(target_name);
            dependencies = line[pos + 1..].trim().split_whitespace().map(String::from).collect();
        } else if let Some(cmd) = line.strip_prefix('\t') {
            if let Some(target) = current_target.as_mut() {
                current_command = Some(cmd.to_string());
            }
        }
    }

    if let Some(target) = current_target {
        targets.insert(target, Target {
            dependencies,
            command: current_command.unwrap_or_default(),
        });
    }

    Ok(targets)
}
