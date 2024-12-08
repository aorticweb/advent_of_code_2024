use anyhow::{anyhow, Context, Result};
use std::collections::HashSet;
use std::fs;
use std::hash::Hash;
use std::path::PathBuf;

#[derive(Clone, Hash, Eq)]
struct Increment {
    row_incr: i32,
    col_incr: i32,
}

impl PartialEq for Increment {
    fn eq(&self, other: &Self) -> bool {
        self.row_incr == other.row_incr && self.col_incr == other.col_incr
    }
}
impl Increment {
    fn new(row_incr: i32, col_incr: i32) -> Self {
        Self { row_incr, col_incr }
    }

    fn increment(&self, row: i32, column: i32) -> (i32, i32) {
        (row + self.row_incr, column + self.col_incr)
    }

    fn unsafe_increment(&self, row: usize, column: usize) -> (usize, usize) {
        (
            (row as i32 + self.row_incr) as usize,
            (column as i32 + self.col_incr) as usize,
        )
    }

    fn from_char(ch: char) -> Option<Self> {
        match ch {
            '^' => Some(Increment::up()),
            'v' => Some(Increment::down()),
            '>' => Some(Increment::right()),
            '<' => Some(Increment::left()),
            _ => None,
        }
    }

    fn up() -> Self {
        Increment::new(-1, 0)
    }

    fn right() -> Self {
        Increment::new(0, 1)
    }

    fn down() -> Self {
        Increment::new(1, 0)
    }

    fn left() -> Self {
        Increment::new(0, -1)
    }

    fn next(&self) -> Increment {
        if *self == Increment::up() {
            return Increment::right();
        } else if *self == Increment::right() {
            return Increment::down();
        } else if *self == Increment::down() {
            return Increment::left();
        } else if *self == Increment::left() {
            return Increment::up();
        }
        println!(
            "unexpected increment values {}, {}",
            self.row_incr, self.col_incr
        );
        Increment::up()
    }
}

#[derive(Clone)]
struct GuardMap {
    obstacles: HashSet<(usize, usize)>,
    max_rows: Option<usize>,
    max_cols: Option<usize>,
    initial_position: Option<(usize, usize)>,
    initial_direction: Option<Increment>,
}

impl GuardMap {
    fn new() -> Self {
        GuardMap {
            obstacles: HashSet::new(),
            max_rows: None,
            max_cols: None,
            initial_position: None,
            initial_direction: None,
        }
    }
    fn is_set(&self) -> bool {
        self.max_rows.is_some()
            && self.max_cols.is_some()
            && self.initial_direction.is_some()
            && self.initial_direction.is_some()
    }

    fn is_within_map(&self, row: i32, column: i32) -> bool {
        row <= self.max_rows.unwrap_or(0) as i32
            && column <= self.max_cols.unwrap_or(0) as i32
            && 0 <= row
            && 0 <= column
    }

    fn is_obstacle(&self, row: usize, column: usize) -> bool {
        self.obstacles.contains(&(row, column))
    }
}

fn read_input(file_path: &PathBuf) -> Result<GuardMap> {
    let input = fs::read_to_string(file_path).context("Failed to read input file")?;
    let mut guard_map = GuardMap::new();
    let mut last_row = 0;
    for (row, line) in input.lines().enumerate() {
        if line.len() == 0 {
            continue;
        }
        for (column, char) in line.chars().enumerate() {
            if char == '#' {
                guard_map.obstacles.insert((row, column));
            }
            if guard_map.initial_position.is_none() {
                match Increment::from_char(char) {
                    Some(increment) => {
                        guard_map.initial_direction = Some(increment);
                        guard_map.initial_position = Some((row, column));
                    }
                    None => {}
                }
            }
        }
        if row == 0 {
            guard_map.max_cols = Some(line.chars().count());
        }
        last_row = row;
    }
    guard_map.max_rows = Some(last_row);
    if guard_map.initial_position.is_none() {
        return Err(anyhow!("did not find initial position of the guard"));
    }
    Ok(guard_map)
}

// returns (whether next location is an obstacle, whether next location is outside the map)
fn checknext_point(
    current_point: (usize, usize),
    current_direction: &Increment,
    guard_map: &GuardMap,
) -> (bool, bool) {
    let next_point = current_direction.increment(current_point.0 as i32, current_point.1 as i32);
    if !guard_map.is_within_map(next_point.0, next_point.1) {
        return (false, true);
    }
    let (next_row, next_col) = (next_point.0 as usize, next_point.1 as usize);
    if guard_map.is_obstacle(next_row, next_col) {
        return (true, false);
    }
    (false, false)
}

fn mark_location_as_visited(
    locations: &mut HashSet<(usize, usize, Increment)>,
    new_location: (usize, usize),
    direction: Increment,
) -> Result<()> {
    if locations.contains(&(new_location.0, new_location.1, direction.clone())) {
        return Err(anyhow!(
            "Stuck in an infinite loop at {}, {}",
            new_location.0,
            new_location.1
        ));
    }
    locations.insert((new_location.0, new_location.1, direction));
    Ok(())
}

// returns the guard path and whether the path is an infinite loop
fn run_guard_path(guard_map: &GuardMap) -> Result<(HashSet<(usize, usize, Increment)>, bool)> {
    // locations visited and the guard direction while visiting it to detect infinite loops
    let mut locations: HashSet<(usize, usize, Increment)> = HashSet::new();
    if !guard_map.is_set() {
        return Err(anyhow!("Map is not properly setup"));
    }
    let mut current_point = guard_map.initial_position.unwrap().clone();
    let mut current_direction = guard_map.initial_direction.as_ref().unwrap().clone();
    let mut infinite_loop_err =
        mark_location_as_visited(&mut locations, current_point, current_direction.clone());

    loop {
        let (is_obstacle, is_outside_map) =
            checknext_point(current_point, &current_direction, &guard_map);
        if is_outside_map {
            return Ok((locations, false));
        }
        if !is_obstacle {
            current_point = current_direction.unsafe_increment(current_point.0, current_point.1);
            infinite_loop_err =
                mark_location_as_visited(&mut locations, current_point, current_direction.clone());
            if infinite_loop_err.is_err() {
                return Ok((locations, true));
            }
            continue;
        }

        let mut found = false;
        for _ in 0..2 {
            current_direction = current_direction.next();
            let (is_obstacle, is_outside_map) =
                checknext_point(current_point, &current_direction, &guard_map);
            if is_outside_map {
                return Ok((locations, false));
            }
            if !is_obstacle {
                current_point =
                    current_direction.unsafe_increment(current_point.0, current_point.1);
                infinite_loop_err = mark_location_as_visited(
                    &mut locations,
                    current_point,
                    current_direction.clone(),
                );
                if infinite_loop_err.is_err() {
                    return Ok((locations, true));
                }
                found = true;
                break;
            }
        }
        if !found {
            return Err(anyhow!(
                "The guard is unexpectadly stuck on the map at ({}, {}), this should not happen",
                current_point.0,
                current_point.1
            ));
        }
    }
}

fn check_is_infinite_loop(mut guard_map: GuardMap, extra_obstacle: (usize, usize)) -> Result<bool> {
    guard_map.obstacles.insert(extra_obstacle);
    let guard_path = run_guard_path(&guard_map)?;
    return Ok(guard_path.1);
}

// Definitively not the most perfmant solution, this is brute force
// we test every unique location, the guard would visit and add an obstacle there
// before re-running the path with the extra obstacle and check if we are stuck in an infinite loop
fn find_infinite_loop_locations(
    guard_map: &GuardMap,
    guard_path: &mut HashSet<(usize, usize)>,
) -> Result<usize> {
    let mut total = 0;
    guard_path.remove(&(
        guard_map.initial_position.unwrap().0,
        guard_map.initial_position.unwrap().1,
    ));
    for location in guard_path.iter() {
        if check_is_infinite_loop(guard_map.clone(), *location)? {
            total += 1
        }
    }
    Ok(total)
}

fn extract_unique_locations(
    guard_path: HashSet<(usize, usize, Increment)>,
) -> HashSet<(usize, usize)> {
    let mut unique_locations: HashSet<(usize, usize)> = HashSet::new();
    for entry in guard_path {
        unique_locations.insert((entry.0, entry.1));
    }
    let mut check: HashSet<(usize, usize)> = HashSet::new();
    for l in unique_locations.iter() {
        if check.contains(&l) {
            println!("dupe found ({}, {})", l.0, l.1);
            continue;
        }
        check.insert(*l);
    }
    unique_locations
}

pub fn solve(file_path: &PathBuf) -> Result<()> {
    let guard_map = read_input(file_path)?;
    let guard_path = run_guard_path(&guard_map)?;
    let mut guard_path_unique_locations = extract_unique_locations(guard_path.0);
    println!(
        "the guard visits {} locations",
        guard_path_unique_locations.len()
    );

    let number_of_potential_infinite_loop =
        find_infinite_loop_locations(&guard_map, &mut guard_path_unique_locations)?;
    println!(
        "the number of potential infinite loop is {}",
        number_of_potential_infinite_loop
    );

    Ok(())
}
