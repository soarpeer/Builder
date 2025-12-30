use std::path::{Path};
use std::fs;

pub fn needs_rebuild(target: &str, dependencies: &[String]) -> bool {
    let target_path = Path::new(target);
    if !target_path.exists() {
        return true; // If the target doesn't exist, it needs to be rebuilt
    }

    for dep in dependencies {
        let dep_path = Path::new(dep);
        if dep_path.exists() {
            let target_metadata = target_path.metadata().unwrap();
            let dep_metadata = dep_path.metadata().unwrap();

            if dep_metadata.modified().unwrap() > target_metadata.modified().unwrap() {
                return true; // If any dependency is newer, rebuild
            }
        }
    }

    false // If no dependencies are newer, don't rebuild
}
