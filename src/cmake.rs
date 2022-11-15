use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::str;
///
/// Validate CMake Lists in passed Dir
/// 
pub fn validate(path: &str) -> bool {
  let cmake_path_string = path.to_owned() + "/CmakeLists.txt";
  let cmake_path = Path::new(&cmake_path_string);

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
    return false;
    //Err("No Exe Or Lib Defined in CmakeLists");
  }

  return true;
}

pub fn get_exe_name(path: &str) -> String {
  if !validate(path) {
    panic!("Invalid CMakeLists");
  }
  let cmake_path_string = path.to_owned() + "/CmakeLists.txt";
  let cmake_path = Path::new(&cmake_path_string);
  
  if let Ok(lines) = read_lines(cmake_path) {
    for line in lines {
      if let Ok(line_string) = line {
        if line_string.starts_with("add_executable") {    
          let split_vector: Vec<&str> = line_string.split(['(', ')', ' ', '\t', '\n']).collect();
          return split_vector[1].to_string();
        }
      }
    }
  }
  return "".to_string();
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}