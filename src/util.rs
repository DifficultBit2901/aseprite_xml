use regex::Regex;

pub fn extract_number(s: &str) -> usize {
    let num_regex = Regex::new(r"\d+").unwrap();
    num_regex
        .find(s)
        .map(|m| m.as_str().parse::<usize>().unwrap_or(0))
        .unwrap_or(0)
}
