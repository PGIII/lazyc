use std::path::Path;
use std::fs;
use std::str;

///
/// Validate CMake Lists in passed Dir
/// 
pub fn validate(path: &str) -> bool {
	let cmake_path_string = path.to_owned() + "/CMakeLists.txt";
	let cmake_path = Path::new(&cmake_path_string);

	//FIXME: check what error we actaully get
	let contents = fs::read_to_string(cmake_path)
		.expect("Error Validating CMake File");
	
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
		//Err("No Exe Or Lib Defined in CMakeLists");
	}

	return true;
}