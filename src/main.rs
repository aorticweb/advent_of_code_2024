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
pub mod day_6;
pub mod day_7;

macro_rules! create_solutions {
    ($($day:expr => $module:ident),* $(,)?) => {{
        let solutions: HashMap<usize, Box<dyn Fn()>> = [
            $(
                (
                    $day,
                    Box::new(|| {
                        $module::solve(&PathBuf::from(concat!("src/day_", $day, "/input.txt")))
                            .expect(&format!("Failed to solve day {}", $day))
                    }) as Box<dyn Fn()>
                ),
            )*
        ]
        .into_iter()
        .collect();
        solutions
    }};
}

fn main() {
    let solutions = create_solutions! {
        1 => day_1,
        2 => day_2,
        3 => day_3,
        4 => day_4,
        5 => day_5,
        6 => day_6,
        7 => day_7,
    };

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
