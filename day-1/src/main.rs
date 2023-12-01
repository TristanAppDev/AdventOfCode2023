use std::fs::read_to_string;

fn main() {
    let lines = read_lines("data.txt");

    let result = lines.into_iter().map(process_line).sum::<u32>();

    println!("{}", result);
}

fn process_line(line: String) -> u32 {
    let mut line = (0..line.len()).filter_map(|index| filter_and_map(&line, index));

    let first = match line.next() {
        Some(n) => n,
        None => 0,
    };

    let last = match line.last() {
        Some(n) => n,
        None => first,
    };

    first * 10 + last
}

fn filter_and_map(line: &str, index: usize) -> Option<u32> {
    let cleaned_line = &line[index..];

    let result = if cleaned_line.starts_with("one") {
        '1'
    } else if cleaned_line.starts_with("two") {
        '2'
    } else if cleaned_line.starts_with("three") {
        '3'
    } else if cleaned_line.starts_with("four") {
        '4'
    } else if cleaned_line.starts_with("five") {
        '5'
    } else if cleaned_line.starts_with("six") {
        '6'
    } else if cleaned_line.starts_with("seven") {
        '7'
    } else if cleaned_line.starts_with("eight") {
        '8'
    } else if cleaned_line.starts_with("nine") {
        '9'
    } else {
        cleaned_line.chars().next().unwrap()
    };
    result.to_digit(10)
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(|s| String::from(s))
        .collect::<Vec<String>>()
}
