//Run should invoke build and then run the exe that was created
use std::{env};
use crate::cmake;
use crate::commands::build;
use crate::terminal;

pub fn execute(preset: &str) {
  build::execute(preset, false);

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
}