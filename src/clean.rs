use std::str;
use std::fs;
use crate::terminal;

pub fn help() {
  println!("Build Usage");
  println!("lazyc clean PRESET[optional]");
}

pub fn handle_command(args: &Vec<String>) {
  
  let preset;
  if args.len() > 2 {
    preset = args[2].as_str();
  } else {
    preset = "default";
  }

  if args[1] == "full-clean" {
    run_full_clean().unwrap();
  } else {
    run_clean(preset);
  }
}

fn run_clean(preset: &str) {
  println!("Cleaning");
  //let build_path = path.join("build");
  let command = format!("cmake --build --preset {} --target clean", preset);
  let output = terminal::run_command(&command);
  let stdout = str::from_utf8(&output.stdout).expect("Error Converting String");
  let stderr = str::from_utf8(&output.stderr).expect("Error Converting String");
  print!("{}", stdout);
  print!("{}", stderr);
}

fn run_full_clean() -> std::io::Result<()> {
  println!("Full Cleaning");
  println!("Removing Build dir");
  return fs::remove_dir_all("build");
}