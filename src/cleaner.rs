use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::collections::HashMap;


pub fn clean(targets: &HashMap<String, crate::parser::Target>) {
    let mut visited = HashSet::new();

    for (target_name, target) in targets {
        if visited.contains(target_name) {
            continue;
        }

        let target_path = Path::new(target_name);
        if target_path.exists() {
            println!("Cleaning: {}", target_name);
            fs::remove_file(target_path).expect("Failed to remove file.");
        }

        visited.insert(target_name);
    }
}
