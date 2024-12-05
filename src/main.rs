use itertools::Itertools;
use std::collections::HashMap;
use std::env;
use std::iter::Iterator;
use std::path::PathBuf;

pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;

fn main() {
    let solutions: HashMap<usize, Box<dyn Fn()>> = [
        (
            1,
            Box::new(|| {
                day_1::solve(PathBuf::from("src/day_1/input.txt")).expect("Failed to solve day one")
            }) as Box<dyn Fn()>,
        ),
        (
            2,
            Box::new(|| {
                day_2::solve(&PathBuf::from("src/day_2/input.txt"))
                    .expect("Failed to solve day two")
            }) as Box<dyn Fn()>,
        ),
        (
            3,
            Box::new(|| {
                day_3::solve(&PathBuf::from("src/day_3/input.txt"))
                    .expect("Failed to solve day three")
            }) as Box<dyn Fn()>,
        ),
        (
            4,
            Box::new(|| {
                day_4::solve(&PathBuf::from("src/day_4/input.txt"))
                    .expect("Failed to solve day four")
            }) as Box<dyn Fn()>,
        ),
        (
            5,
            Box::new(|| {
                day_5::solve(&PathBuf::from("src/day_5/input.txt"))
                    .expect("Failed to solve day four")
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
        Some(day_str) => match day_str.parse::<usize>() {
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
