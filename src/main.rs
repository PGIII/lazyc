use std::env;
use std::path::{PathBuf, Path};
use std::fs::File;
use std::io::prelude::*;
extern crate exitcode;

fn main() {
    //let bytes = include_bytes!("../resources/test.txt");
    //print!("{}", String::from_utf8_lossy(bytes));
    let args: Vec<String> = env::args().collect();
    handle_args(&args);
}

fn handle_args(args: &Vec<String>) {
    let arg_count = args.len();//there will always be one argument
    if arg_count > 1 {
        match args[1].as_str() {
            "new" => handle_new_command(args),
            _ => eprintln!("Error: Unknown Command"),
        };
    } else {
        eprintln!("Error: No Arguments", );
        std::process::exit(exitcode::USAGE);
    }
}

fn handle_new_command(args: &Vec<String>) {
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
        create_new_project(&args[2], path);
    }
}

fn create_new_project(name: &str, pathBuf: PathBuf) -> std::io::Result<()> {
    println!("Creating {} at {}", name, pathBuf.display());
    let path = pathBuf.as_path();
    let testPath = path.join(Path::from("Test"));
    let mut file = File::create(path)?;
    Ok(())
}