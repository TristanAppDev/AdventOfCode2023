use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let digit_map: HashMap<&str, &str> = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let mut result: u32 = 0;

    if let Ok(lines) = read_lines("data.txt".to_owned()) {
        for line in lines {
            if let Ok(cv) = line {
                replace_words_with_nums(digit_map.clone(), cv.clone());
                result += get_val_of_first_and_last(cv)
            }
        }
    }

    println!("{}", result);
}

fn replace_words_with_nums(digit_map: HashMap<&str, &str>, text: String) -> String {
    let mut result: String = String::from("");
    for digit in digit_map {
        result = text.replace(digit.0, digit.1);
    }

    result.to_string()
}

fn get_val_of_first_and_last(line: String) -> u32 {
    return 0;
}

fn read_lines<T>(filename: T) -> io::Result<io::Lines<io::BufReader<File>>>
where
    T: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
