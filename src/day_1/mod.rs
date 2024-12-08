use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

fn read_input(file_path: &PathBuf) -> Result<(Vec<i32>, Vec<i32>, HashMap<i32, i32>)> {
    let input = fs::read_to_string(file_path).context("Failed to read input file")?;
    let mut left_column: Vec<i32> = Vec::new();
    let mut right_column: Vec<i32> = Vec::new();
    let mut right_location_id_map: HashMap<i32, i32> = HashMap::new();

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let left: i32 = parts
            .next()
            .context("Failed to get left column")?
            .parse()
            .context(format!("Failed to parse left column for line: {}", line))?;
        let right: i32 = parts
            .next()
            .context("Failed to get right column")?
            .parse()
            .context(format!("Failed to parse right column for line: {}", line))?;
        left_column.push(left);
        right_column.push(right);
        match right_location_id_map.get_mut(&right) {
            Some(val) => *val += 1,
            None => {
                right_location_id_map.insert(right, 1);
            }
        }
    }
    left_column.sort_unstable();
    right_column.sort_unstable();
    if left_column.len() != right_column.len() {
        return Err(anyhow::anyhow!(format!("left and right column must have the same length, left column has {} elements and right column has {} elements", left_column.len(), right_column.len())));
    }
    Ok((left_column, right_column, right_location_id_map))
}

fn get_sum_diff(left_column: &[i32], right_column: &[i32]) {
    let mut sum_diff = 0;
    left_column
        .iter()
        .zip(right_column.iter())
        .for_each(|(a, b)| {
            sum_diff += a.max(b) - a.min(b);
        });
    println!("Sum of location id differences: {}", sum_diff);
}

fn compute_similarity(left_column: Vec<i32>, right_location_id_map: HashMap<i32, i32>) -> () {
    let mut similarity_score = 0;
    for left in left_column {
        let right_count = right_location_id_map.get(&left).unwrap_or(&0);
        similarity_score += left * right_count;
    }
    println!("Similarity score: {}", similarity_score);
}

pub fn solve(file_path: &PathBuf) -> Result<()> {
    let (left_column, right_column, right_location_id_map) = read_input(file_path)?;
    // Part 1
    get_sum_diff(&left_column, &right_column);
    // Part 2
    compute_similarity(left_column, right_location_id_map);
    Ok(())
}
