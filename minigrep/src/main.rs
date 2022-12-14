use std::{env, error::Error, fs, process};

use minigrep::{args::Args, args_parser, search_engine::search};

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = args_parser::query_and_file_path(&args).unwrap_or_else(|error| {
        println!("Problem passing arguments: {error}");
        process::exit(1);
    });
    println!("Searching for `{}` in `{}`", args.query(), args.file_path());
    if let Err(error) = run(args) {
        println!("Application error: {error}");
        process::exit(1);
    }
}

fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(args.file_path())?;
    for line in search(args.query(), &contents.to_string()) {
        println!("{line}");
    }
    Ok(())
}
