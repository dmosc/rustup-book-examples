pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matches: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(&query) {
            matches.push(line);
        }
    }
    matches
}

pub fn search_lowercase_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut matches: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
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

    #[test]
    pub fn three_results_lowercase_insensitive() {
        let query = "r";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.";
        assert_eq!(
            vec!["Rust:", "safe, fast, productive.", "Pick three."],
            search_lowercase_insensitive(query, contents)
        );
    }

    #[test]
    pub fn no_results_lowercase_insensitive() {
        let query = "Xy";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.";
        assert_eq!(
            vec![] as Vec<&str>,
            search_lowercase_insensitive(query, contents)
        );
    }
}
