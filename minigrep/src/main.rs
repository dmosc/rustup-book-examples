use std::{env, error::Error, fs, process};

use minigrep::{
    args::Args,
    args_parser,
    search_engine::{search, search_lowercase_insensitive},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = args_parser::query_and_file_path(&args).unwrap_or_else(|error| {
        eprintln!("Problem passing arguments: {error}");
        process::exit(1);
    });
    println!("Searching for `{}` in `{}`", args.query(), args.file_path());
    if let Err(error) = run(args) {
        eprintln!("Application error: {error}");
        process::exit(1);
    }
}

fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(args.file_path())?;
    let results = if *args.ignore_case() {
        search_lowercase_insensitive(args.query(), &contents)
    } else {
        search(args.query(), &contents)
    };
    for line in results {
        println!("{line}");
    }
    Ok(())
}
