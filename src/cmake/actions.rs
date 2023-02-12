use std::str;
use crate::terminal;

pub fn configure(path: &str, preset: &str) {
	println!("Running CMake Configure");
	let command = format!("cmake {} --preset {}", path, preset);
	let output = terminal::run_command(&command);
	let stdout = str::from_utf8(&output.stdout).expect("Error Converting String");
	let stderr = str::from_utf8(&output.stderr).expect("Error Converting String");
	print!("{}", stdout);
	print!("{}", stderr);
}