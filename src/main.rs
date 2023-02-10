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
pub enum Commands {
	New {
		#[arg(short, long, help = "Name Of Project To Create")]
		name: String,
		#[arg(short, long, help = "Path To Create Project At", default_value = "./")]
		path: String,
	},
	Build {
		#[arg(short, long, help = "CMake Preset To Use", default_value = "default")]
		preset: String,
	},
	Rebuild {
		#[arg(short, long, help = "CMake Preset To Use", default_value = "default")]
		preset: String,
	},
	Module {
		#[arg(short, long, help = "Name Of Module")]
		name: String,
	},
	Run {
		#[arg(short, long, help = "CMake Preset To Use", default_value = "default")]
		preset: String,
	},
}

fn main() {
	let args = Args::parse();
	new_handle_args(args);
	//handle_args();
}

fn new_handle_args(args: Args) {
	match args.command {
		Commands::New { name, path } => {commands::new::execute(&name, &path)},
		Commands::Build { preset } => {commands::build::execute(&preset, false)},
		Commands::Rebuild { preset } => {commands::build::execute(&preset, true)},
		Commands::Module { name } => {commands::module::create(&name)},
		Commands::Run {preset} => {commands::run::execute(&preset)},
	}
}

fn handle_args() {
	let args: Vec<String> = env::args().collect();
	match env::args().nth(1).expect("Too Few Args").as_str() {
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
			"configure" => commands::configure::help(),
			"full-clean" | "clean" => commands::clean::help(),
			_ => println!("Available Commands: {}", AVAILABLE_COMMANDS.join(", ")),
		}
	}
}