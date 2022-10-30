use std::env;
use std::path::{PathBuf, Path};
use std::fs::File;
use std::io::prelude::*;

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

fn create_new_project(name: &str, path_buf: PathBuf) -> std::io::Result<()> {
  println!("Creating {} at {}", name, path_buf.display());
  let path = path_buf.as_path();

  let cmake_path = path.join(Path::new("CmakeLists.txt"));
  let cmake_file = File::create(cmake_path)?;
  new_project_write_cmake(cmake_file, name)?;
  Ok(())
}

fn new_project_write_cmake(mut file: File, name: &str) -> std::io::Result<()> {
  let bytes = include_bytes!("../resources/CmakeLists_template.cmake");
  let as_string = String::from_utf8_lossy(bytes).to_string();
  let replaced_string = as_string.replace("PLACEHOLDER_PROJECT_NAME", name);
  file.write_all(replaced_string.as_bytes())?;
  Ok(())
}