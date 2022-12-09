use std::env;
mod new;
mod terminal;
mod build;
mod module;
mod run;
mod cmake;
mod configure;
mod clean;

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
			"new" => new::handle_command(args),
			"build" | "rebuild" => build::handle_command(args),
			"help" => handle_help(args),
			"module" => module::handle_command(args),
			"run" => run::handle_command(args),
			"configure" => configure::handle_command(args),
			"full-clean" | "clean" => clean::handle_command(args),
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
			"new" => new::help(),
			"build" | "rebuild" => build::help(),
			"module" => module::help(),
			"run" => run::help(),
			"configure" => configure::help(),
			"full-clean" | "clean" => clean::help(),
			_ => println!("Available Commands: {}", AVAILABLE_COMMANDS.join(", ")),
		}
	}
}