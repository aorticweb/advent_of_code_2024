use anyhow::{Context, Result};
use std::cmp::Ordering;
use std::path::PathBuf;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn convert_str_to_i32(values: Vec<&str>) -> Result<Vec<i32>> {
    values
        .iter()
        .map(|v| {
            v.parse::<i32>()
                .context(format!("Failed to parse value: {}", v))
        })
        .collect::<Result<Vec<i32>>>()
}

// Suppose x = before and y = after
// For a given x we store all the y's that HAVE to come AFTER x
fn add_rule(rules: &mut HashMap<i32, HashSet<i32>>, before: i32, after: i32) {
    if !rules.contains_key(&before) {
        rules.insert(before, HashSet::new());
    }
    rules.get_mut(&before).unwrap().insert(after);
}

fn read_input(file_path: &PathBuf) -> Result<(HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>)> {
    let input = fs::read_to_string(file_path).context("Failed to read input file")?;
    let mut sequences: Vec<Vec<i32>> = Vec::new();
    let mut rules: HashMap<i32, HashSet<i32>> = HashMap::new();

    for line in input.lines() {
        if line.contains("|") {
            let parts: Vec<&str> = line.split("|").collect();
            if parts.len() == 2 {
                let values = convert_str_to_i32(parts)?;
                add_rule(&mut rules, values[0], values[1]);
            }
        } else if line.contains(",") {
            let parts: Vec<&str> = line.split(",").collect();
            sequences.push(convert_str_to_i32(parts)?);
        }
    }
    Ok((rules, sequences))
}

fn process_sequences(
    sequences: Vec<Vec<i32>>,
    rules: &HashMap<i32, HashSet<i32>>,
) -> (i32, Vec<Vec<i32>>) {
    let mut total = 0;
    let mut bad_sequences: Vec<Vec<i32>> = Vec::new();

    for sequence in sequences {
        let mut mid_number = sequence[sequence.len() / 2];
        // check that the number N = sequence[i] is in a correct location
        // i.e in the `rules` the numbers N is supposed to be ahead of (rules[N]) are not already present
        // at indices [0, i - 1]
        for (i, item) in sequence.iter().enumerate() {
            if i == 0 {
                continue;
            }
            let values_supposed_to_come_after = match rules.get(item) {
                Some(values) => values,
                None => {
                    continue;
                }
            };
            let intersection = sequence[0..i]
                .iter()
                .filter(|&&x| values_supposed_to_come_after.contains(&x))
                .collect::<Vec<&i32>>();

            if intersection.len() > 0 {
                bad_sequences.push(sequence);
                mid_number = 0;
                break;
            }
        }
        total += mid_number;
    }
    (total, bad_sequences)
}

fn safe_contains(map: Option<&HashSet<i32>>, value: &i32) -> bool {
    match map {
        Some(m) => m.contains(value),
        None => false,
    }
}

// This is O(n) is unclear here, could be O(n^infinity) but I have to go eat dinner ...
fn sort_sequence(mut sequence: Vec<i32>, rules: &HashMap<i32, HashSet<i32>>) -> Vec<i32> {
    let sort_fn = |a: &i32, b: &i32| {
        match (
            safe_contains(rules.get(a), b),
            safe_contains(rules.get(b), a),
        ) {
            (true, true) => {
                // Consider raising an error
                println!("Rule violation for {} and {}", a, b);
                Ordering::Equal
            }
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            (false, false) => Ordering::Equal,
        }
    };
    // maybe  clone?
    sequence.sort_by(sort_fn);
    sequence
}

// part 2
pub fn re_compute_bad_sequences(
    bad_sequences: Vec<Vec<i32>>,
    rules: &HashMap<i32, HashSet<i32>>,
) -> i32 {
    let mut total = 0;
    for mut sequence in bad_sequences {
        sequence = sort_sequence(sequence, rules);
        total += sequence[sequence.len() / 2];
    }
    total
}

pub fn solve(file_path: &PathBuf) -> Result<()> {
    let (rules, sequences) = read_input(file_path)?;
    let (sum_valid_sequences, bad_sequences) = process_sequences(sequences, &rules);
    println!("The sum of valid sequences is: {}", sum_valid_sequences);
    let sum_invalid_sequences = re_compute_bad_sequences(bad_sequences, &rules);
    println!(
        "The sum of re-sorted invalid sequences is: {}",
        sum_invalid_sequences
    );
    Ok(())
}
