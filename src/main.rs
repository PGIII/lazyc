use std::env;
use clap::{Parser, Subcommand};

mod commands;
mod cmake;
mod terminal;

extern crate exitcode;

const AVAILABLE_COMMANDS: [&str;2] = ["new", "build"];

#[derive(Parser)]
pub struct Args {
	#[command(subcommand)]
	command: Commands, 
}

#[derive(Subcommand)]
enum Commands {
	New {
		#[arg(short, long, help = "Name Of Project To Create")]
		name: String,
		#[arg(short, long)]
		path: String,
	},
	Build {
		#[arg(short, long, help = "CMake Preset To Use")]
		preset: String,
	},
}

fn main() {
	let args = Args::parse();
	handle_args();
}

fn handle_args() {
	let args: Vec<String> = env::args().collect();
	match env::args().nth(1).expect("Too Few Args").as_str() {
		"new" => commands::new::handle_command(&args),
		"build" | "rebuild" => commands::build::handle_command(&args),
		"help" => handle_help(&args),
		"module" => commands::module::handle_command(&args),
		"run" => commands::run::handle_command(&args),
		"configure" => commands::configure::handle_command(&args),
		"full-clean" | "clean" => commands::clean::handle_command(&args),
		_ => eprintln!("Error: Unknown Command"),
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