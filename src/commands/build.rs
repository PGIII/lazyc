use std::str;
use crate::terminal;


pub fn execute(preset: &str, rebuild: bool) {
  if rebuild {
    run_rebuild(preset);
  } else {
    run_build(preset);
  }
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

fn run_rebuild(preset: &str) {
  println!("Building");
  //let build_path = path.join("build");
  let command = format!("cmake --build --clean-first --preset {}", preset);
  let output = terminal::run_command(&command);
  let stdout = str::from_utf8(&output.stdout).expect("Error Converting String");
  let stderr = str::from_utf8(&output.stderr).expect("Error Converting String");
  print!("{}", stdout);
  print!("{}", stderr);
}