pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matched_lines: Vec<&str> = Vec::new();
    for line in contents.split('\n'){
        if line.contains(query) {
            matched_lines.push(line);
        }
    }
    matched_lines
}
