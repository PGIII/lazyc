use crate::{terminal, cmake, commands};
use std::{str, env};

pub fn execute(preset: &str, rebuild: bool) {
    if rebuild {
        run_rebuild(preset);
    } else {
        run_build(preset);
    }
}

fn configure_if_needed(preset: &str) {
    let path = env::current_dir().expect("Error Getting Current Dir");
    let path_string = path
        .into_os_string()
        .into_string()
        .expect("Error Converting Path To String");
    if !cmake::is_configured(&path_string, preset) {
        println!("Not Yet Configured, Running Configure");
        commands::configure::execute(preset);
    }
}

fn run_build(preset: &str) {
    println!("Building");
    configure_if_needed(preset);
    //let build_path = path.join("build");
    let command = format!("cmake --build --preset {}", preset);
    terminal::run_command(&command);
}

fn run_rebuild(preset: &str) {
    println!("Building");
    //let build_path = path.join("build");
    configure_if_needed(preset);
    let command = format!("cmake --build --clean-first --preset {}", preset);
    terminal::run_command(&command);
}
