use std::process::Command;

use rust_dynamic_installation::build::Config;


fn main() {
    let commit_hash = "a3125c7";

    let config = Config::load_config().expect("Could not load config");
    let dependencies = config.dependencies.binaries;
    
    let version = dependencies
        .iter()
        .find(|&t| t.commit_hash == commit_hash)
        .map(|d| d.binary_name.clone())
        .expect("Task not found");


    let bin_path = format!("./bin_dependencies/debcrafter{}", version);
    Command::new(bin_path)
        .status()
        .expect("Failed to execute dependency");
}
