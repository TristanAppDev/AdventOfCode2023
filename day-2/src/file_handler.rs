use std::fs::read_to_string;

pub fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(|s| String::from(s))
        .collect::<Vec<String>>()
}
