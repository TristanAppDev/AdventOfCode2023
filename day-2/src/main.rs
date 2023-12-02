pub mod file_handler;
pub mod game;
pub mod puzzle1;
pub mod puzzle2;
pub mod subset;

use puzzle1::puzzle_one;
use puzzle2::puzzle_two;

fn main() {
    puzzle_one();
    puzzle_two();
}
