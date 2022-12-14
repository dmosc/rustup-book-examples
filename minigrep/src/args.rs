#[derive(Debug, PartialEq)]
pub struct Args {
    query: String,
    file_path: String,
}

impl Args {
    pub fn new(query: String, file_path: String) -> Self {
        Self { query, file_path }
    }

    pub fn query(&self) -> &String {
        &self.query
    }

    pub fn file_path(&self) -> &String {
        &self.file_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_args() {
        let args = Args::new(String::from("query"), String::from("file_path"));
        assert_eq!(args.query(), "query");
        assert_eq!(args.file_path(), "file_path");
    }
}
