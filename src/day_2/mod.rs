use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

fn parse_level(level: &str) -> Result<i32> {
    let parsed_level: i32 = level.parse().context(format!(
        "level value `{}` could not be parsed into an i32",
        level
    ))?;
    Ok(parsed_level)
}

fn get_factor(prev: i32, next: i32) -> Option<i32> {
    (prev - next).checked_div((prev - next).abs())
}

fn report_is_valid(levels: &[i32]) -> bool {
    let factor = get_factor(levels[0], levels[1]);
    if factor.is_none() {
        return false;
    }
    levels
        .iter()
        .zip(levels.iter().skip(1))
        .all(|(prev, next)| {
            let diff = factor.unwrap() * (prev - next);
            0 < diff && diff < 4
        })
}

fn validate_reports(file_path: &PathBuf) -> Result<i32> {
    let input = fs::read_to_string(file_path).context("Failed to read input file")?;
    let mut correct_report_count = 0;
    for line in input.lines() {
        let levels: Vec<&str> = line.split_whitespace().collect();
        let levels: Vec<i32> = levels
            .iter()
            .map(|level| parse_level(level))
            .collect::<Result<Vec<i32>>>()?;
        if levels.len() < 2 {
            correct_report_count += 1;
            continue;
        }
        let factor = get_factor(levels[0], levels[1]);
        if factor.is_none() {
            continue;
        }
        let factor = factor.unwrap();
        let valid_report = levels
            .iter()
            .zip(levels.iter().skip(1))
            .all(|(prev, next)| {
                let diff = factor * (prev - next);
                0 < diff && diff < 4
            });
        if valid_report {
            correct_report_count += 1
        }
    }
    Ok(correct_report_count)
}

// Yes this is O(n^2) ... Sue me...
fn validate_reports_with_problem_dapener(file_path: &PathBuf) -> Result<i32> {
    let input = fs::read_to_string(file_path).context("Failed to read input file")?;
    let mut correct_report_count = 0;
    for line in input.lines() {
        let levels: Vec<&str> = line.split_whitespace().collect();
        let levels: Vec<i32> = levels
            .iter()
            .map(|level| parse_level(level))
            .collect::<Result<Vec<i32>>>()?;
        if levels.len() < 3 {
            correct_report_count += 1;
            continue;
        }

        if report_is_valid(&levels) {
            correct_report_count += 1;
            continue;
        }
        for i in 0..levels.len() {
            let mut lev = levels.clone();
            lev.remove(i);
            if report_is_valid(&lev) {
                correct_report_count += 1;
                break;
            }
        }
    }
    Ok(correct_report_count)
}

pub fn solve(file_path: &PathBuf) -> Result<()> {
    let valid_report_count = validate_reports(file_path)?;
    println!("Number of valid report is: {}", valid_report_count);

    let valid_report_by_problem_dampener = validate_reports_with_problem_dapener(file_path)?;
    println!(
        "Number of valid report when using Problem Dampener methodology is: {}",
        valid_report_by_problem_dampener
    );
    Ok(())
}
