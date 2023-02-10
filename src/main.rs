use std::env;
mod commands;
mod cmake;
mod terminal;

extern crate exitcode;

const AVAILABLE_COMMANDS: [&str;2] = ["new", "build"];
fn main() {
	//let bytes = include_bytes!("../resources/test.txt");
	//print!("{}", String::from_utf8_lossy(bytes));
	let args: Vec<String> = env::args().collect();
	handle_args(&args);
}

fn handle_args(args: &Vec<String>) {
	let arg_count = args.len();//there will always be one argument
	if arg_count > 1 {
		match args[1].to_lowercase().as_str() {
			"new" => commands::new::handle_command(args),
			"build" | "rebuild" => commands::build::handle_command(args),
			"help" => handle_help(args),
			"module" => commands::module::handle_command(args),
			"run" => commands::run::handle_command(args),
			"configure" => commands::configure::handle_command(args),
			"full-clean" | "clean" => commands::clean::handle_command(args),
			_ => eprintln!("Error: Unknown Command"),
		};
	} else {
		eprintln!("Error: No Arguments", );
		std::process::exit(exitcode::USAGE);
	}
}

fn handle_help(args: &Vec<String>) {
	if args.len() < 3 {
		println!("Available Commands: {}", AVAILABLE_COMMANDS.join(", "));
	} else {
		match args[2].as_str().to_lowercase().as_str() {
			"new" => commands::new::help(),
			"build" | "rebuild" => commands::build::help(),
			"module" => commands::module::help(),
			"run" => commands::run::help(),
			"configure" => commands::configure::help(),
			"full-clean" | "clean" => commands::clean::help(),
			_ => println!("Available Commands: {}", AVAILABLE_COMMANDS.join(", ")),
		}
	}
}