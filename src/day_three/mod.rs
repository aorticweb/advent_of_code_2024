use anyhow::{anyhow, Context, Result};
use regex::Regex;
use std::fs;
use std::path::PathBuf;

fn perform_multiplication(mul: String) -> Result<i32> {
    let parts: Vec<&str> = mul.split("(").collect();
    if parts.len() != 2 {
        return Err(anyhow!("Unexpected multiplication format for `{}`", mul));
    }
    let parts: Vec<&str> = parts[1].split(")").collect();
    if parts.len() != 2 {
        return Err(anyhow!("Unexpected multiplication format for `{}`", mul));
    }
    let parts: Vec<&str> = parts[0].split(",").collect();
    let first_num: i32 = parts[0]
        .parse()
        .context("Unexpected multiplication format for `{}`, during number parsing")?;
    let sec_num: i32 = parts[1]
        .parse()
        .context("Unexpected multiplication format for `{}`, during number parsing")?;
    Ok(first_num * sec_num)
}

// Part 1
fn sum_sans_control_flow(input: &str) -> Result<i32> {
    let re = Regex::new(r"mul\(\d{1,3}\,\d{1,3}\)").context("Failed to generate regex")?;
    let mut total = 0;
    let results: Vec<String> = re
        .find_iter(input)
        .map(|mat| mat.as_str().to_string())
        .collect();

    for r in results {
        total += perform_multiplication(r)?;
    }
    Ok(total)
}

// Part 2
fn sum_with_control_flow(input: &str) -> Result<i32> {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|don't|do").context("Failed to generate regex")?;
    let mut total = 0;
    let results: Vec<String> = re
        .find_iter(input)
        .map(|mat| mat.as_str().to_string())
        .collect();
    let mut on_flag = true;
    for r in results {
        if r == "do" {
            on_flag = true;
            continue;
        }
        if r == "don't" {
            on_flag = false;
            continue;
        }
        if on_flag {
            total += perform_multiplication(r)?;
        }
    }
    Ok(total)
}

pub fn solve(file_path: &PathBuf) -> Result<()> {
    println!("Day Three Let's Gooooooo");

    let input = fs::read_to_string(file_path).context("Failed to read input file")?;
    // let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))mul ( 2 , 4 )";
    // let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    let sans_control_flow_total = sum_sans_control_flow(&input)?;

    println!(
        "multiplication sum (Sans Control Flow): {}",
        sans_control_flow_total
    );

    let with_control_flow_total = sum_with_control_flow(&input)?;
    println!(
        "multiplication sum (With Control Flow): {}",
        with_control_flow_total
    );
    Ok(())
}
