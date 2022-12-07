use crate::cmake;
use std::env;

pub fn help() {
	println!("Configure Command Usage");
	println!("lazyc configure PRESET");
}

pub fn handle_command(args: &Vec<String>) {
  let preset;
	if args.len() < 3 {
    //default to ... default
    preset = "default";
	} else {
		preset = &args[2];
	}
  let path = env::current_dir()
		.expect("Couldnt Get Current Dir")
		.into_os_string()
		.into_string()
		.expect("Error Converting Current Path to String");

  cmake::configure(&path, preset);
}