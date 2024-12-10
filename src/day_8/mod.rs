use anyhow::{anyhow, Result};
use itertools::Itertools;
use num::Integer;
use std::collections::HashMap;
use std::path::PathBuf;
use std::{collections::HashSet, fs};

struct Map {
    max_row: i32,
    max_column: i32,
    nodes: HashMap<char, Vec<(i32, i32)>>,
}

impl Map {
    fn is_within_map(&self, point: &(i32, i32)) -> bool {
        0 <= point.0 && point.0 <= self.max_row && 0 <= point.1 && point.1 <= self.max_column
    }
}

fn read_input(file_path: &PathBuf) -> Result<Map> {
    let input = fs::read_to_string(file_path)?;
    let mut nodes: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut max_row: i32 = 0;
    let mut max_col: i32 = 0;
    for (row, line) in input.lines().enumerate() {
        if line.is_empty() {
            continue;
        }
        for (col, char) in line.chars().enumerate() {
            if char == '.' {
                continue;
            }
            if !nodes.contains_key(&char) {
                nodes.insert(char, Vec::new());
            }
            nodes.get_mut(&char).unwrap().push((row as i32, col as i32));
        }
        max_col = (line.len() - 1) as i32;
        max_row = row as i32;
    }
    if max_col == 0 || max_row == 0 {
        return Err(anyhow!("node map is empty"));
    }
    Ok(Map {
        max_row: max_row,
        max_column: max_col,
        nodes: nodes,
    })
}

// part 1 helper
fn find_antinode_for_points_double_spaced(
    point: &(i32, i32),
    other: &(i32, i32),
    map: &Map,
) -> Vec<(i32, i32)> {
    let diff = (point.0 - other.0, point.1 - other.1);
    // Should not happen but just a safety
    if diff.0 == 0 && diff.1 == 1 {
        return vec![];
    }
    let nodes = vec![
        (point.0 + diff.0, point.1 + diff.1),
        (other.0 - diff.0, other.1 - diff.1),
    ];
    nodes
        .iter()
        .filter(|&p| map.is_within_map(p))
        .copied()
        .collect::<Vec<(i32, i32)>>()
}

// part 1 helper
fn get_antinodes_double_spaced(map: &Map) -> usize {
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for nodes in map.nodes.values() {
        nodes
            .iter()
            .combinations(2)
            .map(|pair| find_antinode_for_points_double_spaced(pair[0], pair[1], map))
            .for_each(|points| {
                antinodes.extend(points);
            });
    }
    antinodes.len()
}

// part 2 helper
fn find_points_in_diagonal(point: &(i32, i32), diff: &(i32, i32), map: &Map) -> Vec<(i32, i32)> {
    let mut points: Vec<(i32, i32)> = vec![];
    let mut current_point = *point;
    loop {
        let next_point = (current_point.0 + diff.0, current_point.1 + diff.1);
        if !map.is_within_map(&next_point) {
            break;
        }
        points.push(next_point);
        current_point = next_point;
    }
    points
}

fn make_vector_smaller(diff: (i32, i32)) -> (i32, i32) {
    let gcd = diff.0.gcd(&diff.1);
    (diff.0 / gcd, diff.1 / gcd)
}

// part 2 helper
fn get_antinodes_in_diagonal(point: &(i32, i32), other: &(i32, i32), map: &Map) -> Vec<(i32, i32)> {
    let diff = make_vector_smaller((point.0 - other.0, point.1 - other.1));
    // Should not happen but just a safety
    if diff.0 == 0 && diff.1 == 1 {
        return vec![];
    }
    let mut points: Vec<(i32, i32)> = find_points_in_diagonal(point, &diff, map);
    points.extend(find_points_in_diagonal(point, &(-diff.0, -diff.1), map));
    // need to add the starting point if at least 1 other point was found
    if points.len() > 0 {
        points.push(*point);
    }
    return points;
}

// part 2 helper
fn get_antidoes_by_line(map: &Map) -> usize {
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for nodes in map.nodes.values() {
        nodes
            .iter()
            .combinations(2)
            .map(|pair| get_antinodes_in_diagonal(pair[0], pair[1], map))
            .for_each(|points| {
                antinodes.extend(points);
            });
    }
    antinodes.len()
}

pub fn solve(file_path: &PathBuf) -> Result<()> {
    let map = read_input(file_path)?;
    let antinodes_count_double_spaced = get_antinodes_double_spaced(&map);
    println!(
        "There are {} antinodes for double space rule",
        antinodes_count_double_spaced
    );
    let antinodes_count_line = get_antidoes_by_line(&map);
    println!("There are {} antinodes for line rule", antinodes_count_line);
    Ok(())
}
