pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matched_lines: Vec<&str> = Vec::new();
    for line in contents.lines(){
        if line.contains(query) {
            matched_lines.push(line);
        }
    }
    matched_lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!("safe, fast, productive."), search(query, contents));
    }

    #[test]
    fn no_result(){
        let query = "ducktape";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        let empty_vec: Vec<&str> = vec![];
        assert_eq!(empty_vec, search(query, contents));
    }

    #[test]
    fn multiple_results(){
        let query = "st";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!("Rust:","safe, fast, productive."), search(query, contents));
    }
}
