use std::str;
use crate::terminal;

pub fn help() {
  println!("Build Usage");
  println!("lazyc build PRESET[optional]");
}

pub fn handle_command(args: &Vec<String>) {
  
  let preset;
  if args.len() > 2 {
    preset = args[2].as_str();
  } else {
    preset = "default";
  }
  run_build(preset);
}

fn run_build(preset: &str) {
  println!("Building");
  //let build_path = path.join("build");
  let command = format!("cmake --build --preset {}", preset);
  let output = terminal::run_command(&command);
  let stdout = str::from_utf8(&output.stdout).expect("Error Converting String");
  let stderr = str::from_utf8(&output.stderr).expect("Error Converting String");
  print!("{}", stdout);
  print!("{}", stderr);
}