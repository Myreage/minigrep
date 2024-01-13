pub fn find_matching_lines<'a>(content: &'a String, query: &String) -> Vec<&'a str> {
    let mut matching_lines: Vec<&str> = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            matching_lines.push(line);
        }
    }
    matching_lines
}

#[cfg(test)]
mod test {
    use crate::line_matcher::find_matching_lines;

    #[test]
    fn basic() {
        let content = String::from("Salut\nC'est\nBibi le fou");
        let query = String::from("Bibi");
        let result = find_matching_lines(&content, &query);
        let expected = vec!["Bibi le fou"];
        assert_eq!(result, expected);
    }
}
