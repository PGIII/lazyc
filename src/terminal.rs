use std::process::{Command, Output};
use colored::Colorize;

pub fn run_command(command: &str) -> Output{
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
	let formatted = format!("{}", command).green().bold();
	println!("{}", formatted);
	return output;
}