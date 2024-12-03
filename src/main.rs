pub mod day_one;
pub mod day_three;
pub mod day_two;
use std::path::PathBuf;

fn main() {
    day_one::solve(PathBuf::from("src/day_one/input.txt")).expect("Failed to solve day one");
    day_two::solve(&PathBuf::from("src/day_two/input.txt")).expect("Failed to solve day two");
    day_three::solve(&PathBuf::from("src/day_three/input.txt")).expect("Failed to solve day three");
}
