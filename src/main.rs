use clap::{Parser, Subcommand};

mod commands;
mod cmake;
mod terminal;

extern crate exitcode;

#[derive(Parser)]
pub struct Args {
	#[command(subcommand)]
	command: Commands, 
}

#[derive(Subcommand)]
pub enum Commands {
	/// Create New CMake Project
	New {
		#[arg(short, long, help = "Name Of Project To Create")]
		name: String,
		#[arg(short, long, help = "Path To Create Project At", default_value = "./")]
		path: String,
	},
	/// Build Program
	Build {
		#[arg(short, long, help = "CMake Preset To Use", default_value = "default")]
		preset: String,
	},
	/// Clean And Build Program
	Rebuild {
		#[arg(short, long, help = "CMake Preset To Use", default_value = "default")]
		preset: String,
	},
	/// Create New Module
	Module {
		#[arg(short, long, help = "Name Of Module")]
		name: String,
	},
	/// Build And Run Program
	Run {
		#[arg(short, long, help = "CMake Preset To Use", default_value = "default")]
		preset: String,
	},
	/// Configure With Passed Preset
	Configure {
		#[arg(short, long, help = "CMake Preset To Use", default_value = "default")]
		preset: String,
	},
	/// Run CMake Created clean for Target
	Clean {
		#[arg(short, long, help = "CMake Preset To Use", default_value = "default")]
		preset: String,
	},
	/// Clean All CMake Generated Files
	Fullclean,
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
		Commands::Configure { preset } => {commands::configure::execute(&preset)},
		Commands::Clean { preset } => {commands::clean::run_clean(&preset)},
		Commands::Fullclean => {commands::clean::run_full_clean()}
	}
}