use std::env;
mod new;
mod terminal;

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
            "new" => new::handle_command(args),
            _ => eprintln!("Error: Unknown Command"),
        };
    } else {
        eprintln!("Error: No Arguments", );
        std::process::exit(exitcode::USAGE);
    }
}