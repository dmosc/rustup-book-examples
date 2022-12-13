pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matches: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            matches.push(line);
        }
    }
    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn one_result() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    pub fn two_results() {
        let query = "st";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.";
        assert_eq!(
            vec!["Rust:", "safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    pub fn no_results() {
        let query = "xy";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.";
        assert_eq!(vec![] as Vec<&str>, search(query, contents));
    }
}
