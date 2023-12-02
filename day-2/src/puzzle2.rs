use crate::file_handler::read_lines;
use crate::{game::*, subset};

pub fn puzzle_two() {
    let games = read_lines("data.txt")
        .into_iter()
        .map(|line| parse_line_to_game(&line))
        .collect::<Vec<Game>>();

    let result = games
        .into_iter()
        .map(|game| get_max_color_of_game(game))
        .map(|color| multiply_colors(color))
        .sum::<u32>();

    println!("{}", result);
}

fn get_max_color_of_game(game: Game) -> (u32, u32, u32) {
    let mut highest_red = 0;
    let mut highest_green = 0;
    let mut highest_blue = 0;

    for subset in game.subsets {
        if subset.red_cubes > highest_red {
            highest_red = subset.red_cubes;
        }
        if subset.green_cubes > highest_green {
            highest_green = subset.green_cubes;
        }
        if subset.blue_cubes > highest_blue {
            highest_blue = subset.blue_cubes;
        }
    }

    (highest_red, highest_green, highest_blue)
}

fn multiply_colors(colors: (u32, u32, u32)) -> u32 {
    colors.0 * colors.1 * colors.2
}
