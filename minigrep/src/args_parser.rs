use crate::config::Config;

pub fn query_and_file_path(args: &Vec<String>) -> Result<Config, &'static str> {
    if args.len() < 3 {
        Err("not enough arguments")
    } else {
        let query = args.get(1).unwrap().to_string();
        let file_path = args.get(2).unwrap().to_string();
        Ok(Config::new(query, file_path))
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
            Config::new(String::from("query"), String::from("file_path"))
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
