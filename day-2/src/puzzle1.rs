use std::fs::read_to_string;
use std::iter::Sum;

struct Subset {
    red_cubes: u32,
    green_cubes: u32,
    blue_cubes: u32,
}

struct Game {
    id: u32,
    subsets: Vec<Subset>,
}

pub fn puzzle_one() {
    let limit = Subset {
        red_cubes: 12,
        green_cubes: 13,
        blue_cubes: 14,
    };

    let games = read_lines("data.txt")
        .into_iter()
        .map(|line| parse_line_to_game(&line))
        .collect::<Vec<Game>>();

    let possible_games = games
        .into_iter()
        .filter(|game| is_game_possible(game, &limit))
        .collect::<Vec<Game>>();

    let result = possible_games
        .into_iter()
        .fold(0, |acc: u32, game| acc + game.id);

    println!("{}", result);
}

fn is_game_possible(game: &Game, limit: &Subset) -> bool {
    for subset in &game.subsets {
        if subset.red_cubes > limit.red_cubes
            || subset.green_cubes > limit.green_cubes
            || subset.blue_cubes > limit.blue_cubes
        {
            return false;
        }
    }
    true
}

fn parse_line_to_game(line: &str) -> Game {
    Game {
        id: get_id_from_line(line),
        subsets: parse_records_to_subsets(line),
    }
}

fn get_id_from_line(line: &str) -> u32 {
    let id = match line.split(":").next() {
        Some(s) => s.trim().split(" ").last().unwrap(),
        None => "0",
    };
    println!("{}", id);

    return id.parse::<u32>().unwrap();
}

fn parse_records_to_subsets(line: &str) -> Vec<Subset> {
    line.split(":")
        .last()
        .unwrap()
        .split(";")
        .map(|record| parse_record_to_subset(record))
        .collect::<Vec<Subset>>()
}

fn parse_record_to_subset(record: &str) -> Subset {
    let colors = record.split(",").map(|c| c.trim()).collect::<Vec<&str>>();

    let mut red_cubes: u32 = 0;
    let mut green_cubes: u32 = 0;
    let mut blue_cubes: u32 = 0;

    for color in colors {
        let count = color.split(" ").next().unwrap().parse::<u32>().unwrap();
        let name = color.split(" ").last().unwrap();

        match name {
            "red" => red_cubes = count,
            "green" => green_cubes = count,
            _ => blue_cubes = count,
        }
    }

    Subset {
        red_cubes,
        green_cubes,
        blue_cubes,
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(|s| String::from(s))
        .collect::<Vec<String>>()
}
