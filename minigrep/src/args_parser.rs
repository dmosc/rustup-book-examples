use std::env;

use crate::args::Args;

pub fn query_and_file_path(mut args: impl Iterator<Item = String>) -> Result<Args, &'static str> {
    args.next();
    let query = match args.next() {
        Some(arg) => arg,
        None => return Err("expecting query string"),
    };
    let file_path = match args.next() {
        Some(arg) => arg,
        None => return Err("expecting file path string"),
    };
    let ignore_args = env::var("IGNORE_CASE").is_ok();
    Ok(Args::new(query, file_path, ignore_args))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_query_and_file_path() {
        let args = vec![
            String::from("target/debug/minigrep"),
            String::from("query"),
            String::from("file_path"),
        ];
        let parsed_args = query_and_file_path(args.into_iter()).unwrap();
        assert_eq!(
            parsed_args,
            Args::new(String::from("query"), String::from("file_path"), false)
        );
    }

    #[test]
    #[should_panic(expected = "expecting query string")]
    fn no_query_arg() {
        let args = vec![String::from("target/debug/minigrep")];
        query_and_file_path(args.into_iter()).unwrap();
    }

    #[test]
    #[should_panic(expected = "expecting file path string")]
    fn no_file_path_arg() {
        let args = vec![String::from("target/debug/minigrep"), String::from("query")];
        query_and_file_path(args.into_iter()).unwrap();
    }
}
