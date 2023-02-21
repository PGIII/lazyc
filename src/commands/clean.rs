use std::str;
use std::fs;
use crate::terminal;

pub fn run_clean(preset: &str) {
  println!("Cleaning");
  //let build_path = path.join("build");
  let command = format!("cmake --build --preset {} --target clean", preset);
  terminal::run_command(&command);
}

pub fn run_full_clean() {
  println!("Full Cleaning");
  println!("Removing Build dir");
  fs::remove_dir_all("build").expect("Cant Remove Dir");
}