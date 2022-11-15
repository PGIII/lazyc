use std::env;
use std::fs;
use std::path::Path;
use std::str;

pub fn help() {
  println!("Module Command Usage");
  println!("lazyc module MODULE_NAME");
}

pub fn handle_command() {
  create_new_module();
}

fn create_new_module() {
  //First make sure we are in a dir with a lib or exe CMakeLists
  if validate_cmake_lists() {
    println!("Valid CMake");
    //now insert module c file into target_sources
  } else {
    eprintln!("Invalid CMake");
    std::process::exit(exitcode::OSFILE);        
  }
}

fn insert_into_target_sources() {

}

fn validate_cmake_lists() -> Result<String,std::io::Error> {
  let path = env::current_dir()
    .expect("Couldnt Get Current Dir");
  let cmake_path = path.join(Path::new("CmakeLists.txt"));

  //FIXME: check what error we actaully get
  let contents = fs::read_to_string(cmake_path)
    .expect("Error Validating Cmake File");
  
  //remove_whitespace(&mut contents); 
  let content_lines = str::lines(&contents);
  let mut has_exe = false;
  let mut has_lib = false;
  for line in content_lines {
    if line.starts_with("add_executable"){
      has_exe = true;
      break;
    } else if line.starts_with("add_library") {
      has_lib = true;
      break;
    }
  }  

  if !has_exe && !has_lib {
    Err("No Exe Or Lib Defined in CmakeLists");
  }

  Ok("Blah");
}