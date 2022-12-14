//Run should invoke build and then run the exe that was created
use std::{env};
use crate::cmake;
use crate::build;
use crate::terminal;

pub fn help() {
  println!("Run Function builds and runs executable");
}

pub fn handle_command(args: &Vec<String>) {
  let preset;
  //if no args assume its in build/default/release
  if args.len() < 3 {
    preset = "default";
  } else {
    preset = &args[2];
  }

  build::handle_command(&vec!["internal".to_string(), "build".to_string(), preset.to_string()]);

  let path = env::current_dir()
    .expect("Error Getting Current Dir");
  let path_string = path.into_os_string().into_string()
    .expect("Error Converting Path To String");

  let bin_dir = cmake::get_preset_binary_dir(&path_string, preset).expect("Error Finding Exe");
  let exe = bin_dir;
  let exe_name = cmake::get_exe_name(&path_string);
  let command = exe + "/" + &exe_name;
  let mut child_process = terminal::spawn_command(&command);
  child_process.wait().unwrap();//if we error out panic
  // let output = terminal::run_command(&command);
  // let stdout = str::from_utf8(&output.stdout).expect("Error Converting String");
  // let stderr = str::from_utf8(&output.stderr).expect("Error Converting String");
  // print!("{}", stdout);
  // print!("{}", stderr);
}