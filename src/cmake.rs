use crate::terminal;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead, ErrorKind};
use std::path::Path;
use std::str;

///
/// Validate CMake Lists in passed Dir
///
pub fn validate(path: &str) -> bool {
    let cmake_path_string = path.to_owned() + "/CMakeLists.txt";
    let cmake_path = Path::new(&cmake_path_string);

    //FIXME: check what error we actaully get
    let contents = fs::read_to_string(cmake_path).expect("Error Validating CMake File");

    //remove_whitespace(&mut contents);
    let content_lines = str::lines(&contents);
    let mut has_exe = false;
    let mut has_lib = false;
    for line in content_lines {
        if line.starts_with("add_executable") {
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

pub fn get_exe_name(path: &str) -> String {
    if !validate(path) {
        panic!("Invalid CMakeLists");
    }
    let cmake_path_string = path.to_owned() + "/CMakeLists.txt";
    let cmake_path = Path::new(&cmake_path_string);

    if let Ok(lines) = read_lines(cmake_path) {
        for line in lines {
            if let Ok(line_string) = line {
                if line_string.starts_with("add_executable") {
                    let split_vector: Vec<&str> =
                        line_string.split(['(', ')', ' ', '\t', '\n']).collect();
                    return split_vector[1].to_string();
                }
            }
        }
    }
    return "".to_string();
}

pub fn add_to_target_sources(path: &str, target_name: &str, new_source_file_path: &str) {
    //First Read File into Buffer
    let cmake_path_string = path.to_owned() + "/CMakeLists.txt";
    let cmake_path = Path::new(&cmake_path_string);

    //FIXME: check what error we actaully get
    let contents = fs::read_to_string(cmake_path).expect("Error Reading CMake File");

    let content_lines = str::lines(&contents);
    let sources_string = format!("target_sources({}", target_name);
    let mut new_content_lines = "".to_string();
    for line in content_lines {
        let stripped_line: String = strip_string(line, "( "); //remove possible leading space to name
        if stripped_line.starts_with(&sources_string) {
            //now we need to add after the target name
            let with_out_start: String = line.split(&sources_string).collect();
            let new_start = format!(
                "{} \r\n\tPUBLIC {}\r\n",
                sources_string, new_source_file_path
            ); //FIXME: Need to have an option for interface, public, or private
            let new_line = new_start + &with_out_start;
            new_content_lines += &new_line;
        } else {
            new_content_lines += line;
            new_content_lines += "\r\n";
        }
    }
    fs::write(cmake_path, new_content_lines).expect("Error Writing Out CMakeLists");
}

pub fn configure(path: &str, preset: &str) {
    println!("Running CMake Configure");
    let command = format!("cmake {} --preset {}", path, preset);
    terminal::run_command(&command);
}

pub fn get_preset_binary_dir(path: &str, preset: &str) -> Result<String, io::Error> {
    let full_path = path.to_string() + "/CMakePresets.json";
    let file = fs::read_to_string(&full_path).expect("Error Opening Preset File");
    let result = json::parse(&file).expect("Error Parsing Presets File");
    for i in 0..result["configurePresets"].len() {
        if result["configurePresets"][i]["name"] == preset {
            let mut dir = result["configurePresets"][i]["binaryDir"].to_string();
            dir = dir.replace("${sourceDir}", path);
            return Ok(dir);
        }
    }
    return Err(io::Error::from(ErrorKind::NotFound));
}

pub fn is_configured(path: &str, preset: &str) -> bool {
    //easiest way to check for config is see if build dir specified by preset exists
    let dir = get_preset_binary_dir(path, preset);
    match dir {
        Ok(dir) => {
            return Path::new(&dir).exists();
        },
        Err(_) => {
            return false;
        }
    }

}
fn strip_string(string: &str, string_to_remove: &str) -> String {
    //split on string then collect
    let new_string: String = string.split(string_to_remove).collect();
    return new_string;
}

fn _remove_whitespace(string: &str) -> String {
    return string.chars().filter(|c| !c.is_whitespace()).collect();
}
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
