use itertools::Itertools;
use std::collections::HashMap;
use std::env;
use std::iter::Iterator;
use std::path::PathBuf;

pub mod day_1;
pub mod day_2;
pub mod day_3;

fn main() {
    let solutions: HashMap<i32, Box<dyn Fn()>> = [
        (
            1_i32,
            Box::new(|| {
                day_1::solve(PathBuf::from("src/day_1/input.txt")).expect("Failed to solve day one")
            }) as Box<dyn Fn()>,
        ),
        (
            2_i32,
            Box::new(|| {
                day_2::solve(&PathBuf::from("src/day_2/input.txt"))
                    .expect("Failed to solve day two")
            }) as Box<dyn Fn()>,
        ),
        (
            3_i32,
            Box::new(|| {
                day_3::solve(&PathBuf::from("src/day_3/input.txt"))
                    .expect("Failed to solve day three")
            }) as Box<dyn Fn()>,
        ),
    ]
    .into_iter()
    .collect();

    let args: Vec<String> = env::args().collect();
    match args
        .iter()
        .position(|arg| arg == "--day")
        .and_then(|i| args.get(i + 1))
    {
        Some(day_str) => match day_str.parse::<i32>() {
            Ok(day) => {
                if let Some(solve) = solutions.get(&day) {
                    solve();
                } else {
                    println!("Challenge for day: {} has not been solved yet :'(", day);
                }
            }
            Err(_) => println!("Invalid day number: `{}`", day_str),
        },
        None => solutions
            .iter()
            .sorted_by_key(|(&k, _)| k)
            .for_each(|(k, solve)| {
                println!("{}", "-".repeat(30));
                println!("{}", "-".repeat(30));
                println!("Solving challenge for day: {}", k);
                solve()
            }),
    }
}
