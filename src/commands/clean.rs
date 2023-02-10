use std::str;
use std::fs;
use crate::terminal;

pub fn run_clean(preset: &str) {
  println!("Cleaning");
  //let build_path = path.join("build");
  let command = format!("cmake --build --preset {} --target clean", preset);
  let output = terminal::run_command(&command);
  let stdout = str::from_utf8(&output.stdout).expect("Error Converting String");
  let stderr = str::from_utf8(&output.stderr).expect("Error Converting String");
  print!("{}", stdout);
  print!("{}", stderr);
}

pub fn run_full_clean() {
  println!("Full Cleaning");
  println!("Removing Build dir");
  fs::remove_dir_all("build").expect("Cant Remove Dir");
}