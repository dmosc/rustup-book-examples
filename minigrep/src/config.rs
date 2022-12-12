#[derive(Debug, PartialEq)]
pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
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
    fn create_config() {
        let config = Config::new(String::from("query"), String::from("file_path"));
        assert_eq!(config.query(), "query");
        assert_eq!(config.file_path(), "file_path");
    }
}
