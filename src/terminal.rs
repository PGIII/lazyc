use std::process::{Command, Output, Child};
use colored::Colorize;

pub fn run_command(command: &str) -> Output{
	let formatted = format!("{}", command).green().bold();
	println!("{}", formatted);
	let output = if cfg!(target_os = "windows") {
		Command::new("cmd")
						.args(["/C", command])
						.output()
						.expect("failed to execute process")
	} else {
		Command::new("sh")
						.arg("-c")
						.arg(command)
						.output()
						.expect("failed to execute process")
	};
	return output;
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