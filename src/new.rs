use std::{env, str};
use std::path::{PathBuf, Path};
use std::fs::{File, create_dir_all};
use std::io::prelude::*;
use crate::terminal;

pub fn handle_command(args: &Vec<String>) {
  if args.len() < 3 {
	  eprintln!("Error CMD New: Project name not supplied", );
	  std::process::exit(exitcode::USAGE);        
  } else {
	  let path;
	  if args.len() >= 4 {
		  path = PathBuf::from(&args[3]);
	  } else {
		  path = env::current_dir().expect("Couldnt Get Current Dir");
	  }
	  println!("Creating Project: {}", args[2]);
	  create_new_project(&args[2], path).expect("Error Creating Project");
  }
}

pub fn help() {
	println!("New Command Usage");
	println!("lazyc new PROJECT_NAME PROJECT_PATH[optional]");
}

fn create_dirs(path: &Path) -> std::io::Result<()> {
	create_dir_all(path.join("src"))?;
	create_dir_all(path.join("build"))?;
	create_dir_all(path.join("lib"))?;
	Ok(())
}

fn create_new_project(name: &str, path_buf: PathBuf) -> std::io::Result<()> {
	println!("Creating {} at {}", name, path_buf.display());
	let path = path_buf.as_path();

	create_dirs(path)?;
	new_project_write_cmake(path, name)?;
	write_main_c(path)?;
	configure_cmake(path);
	Ok(())
}

fn write_main_c(path: &Path) -> std::io::Result<()> {
	let mut main_file = File::create(path.join(Path::new("src/main.c")))?;
	let main_bytes = include_bytes!("../resources/main_template.c");
	main_file.write_all(main_bytes)?;
	Ok(())
}

fn new_project_write_cmake(path: &Path, name: &str) -> std::io::Result<()> {
	let mut cmake_file = File::create(path.join(Path::new("CMakeLists.txt")))?;
	let bytes = include_bytes!("../resources/CMakeLists_template.cmake");
	let as_string = String::from_utf8_lossy(bytes).to_string();
	let replaced_string = as_string.replace("PLACEHOLDER_PROJECT_NAME", name);
	cmake_file.write_all(replaced_string.as_bytes())?;

	let mut cmake_presets_file = File::create(path.join(Path::new("CMakePresets.json")))?;
	let presets_bytes = include_bytes!("../resources/CMakePresets_template.json");
	cmake_presets_file.write_all(presets_bytes)?;
	Ok(())
}

fn configure_cmake(path: &Path) {
	println!("Running CMake Configure");
	let command = format!("cmake {} --preset default", path.to_string_lossy());
	let output = terminal::run_command(&command);
	let stdout = str::from_utf8(&output.stdout).expect("Error Converting String");
	let stderr = str::from_utf8(&output.stderr).expect("Error Converting String");
	print!("{}", stdout);
	print!("{}", stderr);
}