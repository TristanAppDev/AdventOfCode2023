use crate::file_handler::read_lines;
use crate::game::*;
use crate::subset::Subset;

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
