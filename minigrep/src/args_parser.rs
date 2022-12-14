use std::env;

use crate::args::Args;

pub fn query_and_file_path(args: &Vec<String>) -> Result<Args, &'static str> {
    if args.len() < 3 {
        Err("not enough arguments")
    } else {
        let query = args.get(1).unwrap().to_string();
        let file_path = args.get(2).unwrap().to_string();
        let ignore_args = env::var("IGNORE_CASE").is_ok();
        Ok(Args::new(query, file_path, ignore_args))
    }
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
        let parsed_args = query_and_file_path(&args).unwrap();
        assert_eq!(
            parsed_args,
            Args::new(String::from("query"), String::from("file_path"), false)
        );
    }

    #[test]
    #[should_panic(expected = "not enough arguments")]
    fn no_query_arg() {
        let args = vec![String::from("target/debug/minigrep")];
        query_and_file_path(&args).unwrap();
    }

    #[test]
    #[should_panic(expected = "not enough arguments")]
    fn no_file_path_arg() {
        let args = vec![String::from("target/debug/minigrep"), String::from("query")];
        query_and_file_path(&args).unwrap();
    }
}
