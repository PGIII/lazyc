use colored::Colorize;
use std::process::{Child, Command};
/// Runs command outputting stdout and stderr through parent
/// panics if command returns error status code
pub fn run_command(command: &str) {
    let formatted = format!("{}", command).green().bold();
    println!("{}", formatted);
    let status = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", command])
            .status()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .status()
            .expect("failed to execute process")
    };

    assert!(status.success());
}

pub fn spawn_command(command: &str) -> Child {
    let formatted = format!("{}", command).green().bold();
    println!("{}", formatted);
    return Command::new("sh")
        .arg("-c")
        .arg(command)
        .spawn()
        .expect("Error Spawning Process");
}
