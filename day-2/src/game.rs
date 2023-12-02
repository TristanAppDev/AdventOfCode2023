use crate::subset::Subset;

pub struct Game {
    pub id: u32,
    pub subsets: Vec<Subset>,
}

pub fn parse_line_to_game(line: &str) -> Game {
    Game {
        id: get_id_from_line(line),
        subsets: parse_records_to_subsets(line),
    }
}

pub fn get_id_from_line(line: &str) -> u32 {
    let id = match line.split(":").next().unwrap().trim().split(" ").last() {
        Some(s) => s,
        None => "0",
    };
    println!("{}", id);

    return id.parse::<u32>().unwrap();
}

pub fn parse_records_to_subsets(line: &str) -> Vec<Subset> {
    line.split(":")
        .last()
        .unwrap()
        .split(";")
        .map(|record| parse_record_to_subset(record))
        .collect::<Vec<Subset>>()
}

pub fn parse_record_to_subset(record: &str) -> Subset {
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
