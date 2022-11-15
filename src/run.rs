//Run should invoke build and then run the exe that was created
use std::{env, str};
use crate::cmake;
use crate::build;
use crate::terminal;

pub fn help() {
  println!("Run Function builds and runs executable");
}

pub fn handle_command(_args: &Vec<String>) {
  build::handle_command(&vec!["".to_string()]);

  //if no args assume its in build/default/release
  let path = env::current_dir()
    .expect("Error Getting Current Dir");
  let path_string = path.into_os_string().into_string()
    .expect("Error Converting Path To String");

  let exe_name = cmake::get_exe_name(&path_string);
  let command = "./build/default/release/".to_string() + &exe_name;
  let output = terminal::run_command(&command);
  let stdout = str::from_utf8(&output.stdout).expect("Error Converting String");
  let stderr = str::from_utf8(&output.stderr).expect("Error Converting String");
  print!("{}", stdout);
  print!("{}", stderr);
}