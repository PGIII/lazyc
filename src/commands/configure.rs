use crate::cmake;
use std::env;

pub fn execute(preset: &str) {
  let path = env::current_dir()
		.expect("Couldnt Get Current Dir")
		.into_os_string()
		.into_string()
		.expect("Error Converting Current Path to String");

  cmake::configure(&path, preset);
}