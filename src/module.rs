use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path};
use crate::cmake;

pub fn help() {
	println!("Module Command Usage");
	println!("lazyc module MODULE_NAME");
}

pub fn handle_command(args: &Vec<String>) {
	if args.len() < 3 {
		panic!("Error CMD module: module name not supplied");
	} else {
		create_new_module(&args[2]);
	}
}

fn create_new_module(module_name: &str) {
	//First make sure we are in a dir with a lib or exe CMakeLists
	let working_path = env::current_dir()
		.expect("Couldnt Get Current Dir")
		.into_os_string()
		.into_string()
		.expect("Error Converting Current Path to String");

	if cmake::validate(&working_path) {
		let target_name = cmake::get_exe_name(&working_path);
		let src_path = format!("src/{module_name}.c");
		cmake::add_to_target_sources(&working_path, &target_name, &src_path);
		create_module_files(&working_path, module_name);
		//now insert module c file into target_sources
	} else {
		eprintln!("Invalid CMake");
		std::process::exit(exitcode::OSFILE);        
	}
}

fn create_module_files(path: &str, module_name: &str) {
	let source_path = format!("{}/src/{}.c", path, module_name);
	let header_path = format!("{}/src/{}.h", path, module_name);

	let mut src_file = File::create(Path::new(&source_path))
		.expect("Error Creating Module Source File");
	let mut header_file = File::create(Path::new(&header_path))
		.expect("Error Creating Module Header File");

	let src_temp_bytes = include_bytes!("../resources/module_template.c");
	let header_temp_bytes = include_bytes!("../resources/module_template.h");
	let mut src_temp_str = String::from_utf8_lossy(src_temp_bytes).to_string();
	let mut header_temp_str = String::from_utf8_lossy(header_temp_bytes).to_string();
	src_temp_str = src_temp_str.replace("PLACEHOLDER_MODULE_NAME", module_name);
	header_temp_str = header_temp_str.replace("PLACEHOLDER_MODULE_NAME", module_name);

	src_file.write_all(src_temp_str.as_bytes())
		.expect("Error Writing Module Src");
	header_file.write_all(header_temp_str.as_bytes())
		.expect("Error Writing Module Header");
}