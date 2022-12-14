#[derive(Debug, PartialEq)]
pub struct Args {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Args {
    pub fn new(query: String, file_path: String, ignore_case: bool) -> Self {
        Self {
            query,
            file_path,
            ignore_case,
        }
    }

    pub fn query(&self) -> &String {
        &self.query
    }

    pub fn file_path(&self) -> &String {
        &self.file_path
    }

    pub fn ignore_case(&self) -> &bool {
        &self.ignore_case
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_args() {
        let args = Args::new(String::from("query"), String::from("file_path"), false);
        assert_eq!(args.query(), "query");
        assert_eq!(args.file_path(), "file_path");
    }
}
