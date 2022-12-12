use std::{env, error::Error, fs, process};

use minigrep::{args_parser, config::Config};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = args_parser::query_and_file_path(&args).unwrap_or_else(|error| {
        println!("Problem passing arguments: {error}");
        process::exit(1);
    });
    println!(
        "Searching for `{}` in `{}`",
        config.query(),
        config.file_path()
    );
    if let Err(error) = run(config) {
        println!("Application error: {error}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path())?;
    println!("{contents}");
    Ok(())
}
