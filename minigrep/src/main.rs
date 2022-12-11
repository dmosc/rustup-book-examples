use std::{env, fs};

use minigrep::args_parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = args_parser::query_and_file_path(&args).unwrap();

    println!(
        "Searching for `{}` in `{}`",
        config.query(),
        config.file_path()
    );
    let contents = fs::read_to_string(config.file_path()).expect("Couldn't load file contents");
    println!("{contents}");
}
