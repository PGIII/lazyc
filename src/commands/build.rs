use crate::terminal;
use std::str;

pub fn execute(preset: &str, rebuild: bool) {
    if rebuild {
        run_rebuild(preset);
    } else {
        run_build(preset);
    }
}

fn run_build(preset: &str) {
    println!("Building");
    //let build_path = path.join("build");
    let command = format!("cmake --build --preset {}", preset);
    terminal::run_command(&command);
}

fn run_rebuild(preset: &str) {
    println!("Building");
    //let build_path = path.join("build");
    let command = format!("cmake --build --clean-first --preset {}", preset);
    terminal::run_command(&command);
}
