use anyhow::{anyhow, Context, Result};
use std::fs;
use std::path::PathBuf;

// Read text into a 2D matrix
fn read_input(file_path: &PathBuf) -> Result<Vec<Vec<char>>> {
    let input = fs::read_to_string(file_path).context("Failed to read input file")?;
    let mut rv: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        if line.len() == 0 {
            continue;
        }
        let mut line_vec = Vec::new();
        for char in line.chars() {
            line_vec.push(char);
        }
        // Let's verify that the matrix is a rectangle
        if rv.len() > 0 && rv[0].len() != line_vec.len() {
            return Err(anyhow!(
                "row does not have the expected number of letters, expected {}, got {}",
                rv.len(),
                line_vec.len()
            ));
        }
        rv.push(line_vec);
    }
    Ok(rv)
}

struct Coordinate {
    row: usize,
    column: usize,
}

impl Coordinate {
    fn new(row: usize, col: usize) -> Self {
        Self {
            row: row,
            column: col,
        }
    }
}

struct PathLine {
    coordinates: Vec<Coordinate>,
}

impl PathLine {
    fn skip(&self, word_len: usize) -> bool {
        self.coordinates.len() < word_len
    }
}

fn generate_line_paths(rows: usize, columns: usize) -> Vec<PathLine> {
    let mut paths: Vec<PathLine> = Vec::new();

    // Horizontal traversal
    for row in 0..rows {
        let mut row_coords: Vec<Coordinate> = Vec::new();
        for column in 0..columns {
            row_coords.push(Coordinate::new(row, column));
        }
        paths.push(PathLine {
            coordinates: row_coords,
        });
    }

    // Vertical traversal
    for column in 0..columns {
        let mut col_coords: Vec<Coordinate> = Vec::new();
        for row in 0..rows {
            col_coords.push(Coordinate::new(row, column));
        }
        paths.push(PathLine {
            coordinates: col_coords,
        });
    }

    // TODO:
    // We should unify the loops for both diagonals
    // Traverse diagonal from top-left to bottom-right
    for d in 0..(rows + columns - 1) {
        let mut diag_coords: Vec<Coordinate> = Vec::new();
        for i in 0..=d {
            let row = i;
            let col = d - i;
            if row < rows && col < columns {
                diag_coords.push(Coordinate::new(row, col));
            }
        }
        if !diag_coords.is_empty() {
            paths.push(PathLine {
                coordinates: diag_coords,
            });
        }
    }

    // Traverse diagonal from top-right to bottom-left
    for d in 0..(rows + columns - 1) {
        let mut diag_coords: Vec<Coordinate> = Vec::new();
        for i in 0..=d {
            let row = i;
            if d - i < columns {
                let col = columns - 1 - (d - i);
                if row < rows {
                    diag_coords.push(Coordinate::new(row, col));
                }
            }
        }
        if !diag_coords.is_empty() {
            paths.push(PathLine {
                coordinates: diag_coords,
            });
        }
    }
    paths
}

fn find_word_count_in_path(
    matrix: &Vec<Vec<char>>,
    coordinates: &[&Coordinate],
    word_chars: &[char],
) -> i32 {
    let mut total = 0;
    let mut agg: Vec<char> = Vec::new();
    let mut word_index: usize = 0;

    for coord in coordinates {
        if matrix[coord.row][coord.column] == word_chars[word_index] {
            agg.push(matrix[coord.row][coord.column]);
            word_index += 1;
        } else if matrix[coord.row][coord.column] == word_chars[0] {
            agg = vec![matrix[coord.row][coord.column]];
            word_index = 1;
        } else {
            agg = Vec::new();
            word_index = 0;
        }
        if agg == word_chars {
            total += 1;
            // reset tracking
            agg = Vec::new();
            word_index = 0;
        }
    }
    total
}

fn find_word_count_in_matrix(matrix: &Vec<Vec<char>>, paths: Vec<PathLine>, word: &str) -> i32 {
    let mut total = 0;
    let word_chars: Vec<char> = word.chars().collect();
    let word_chars_reverse = word_chars.iter().rev().copied().collect::<Vec<_>>();
    for path in paths {
        if path.skip(word_chars.len()) {
            continue;
        }
        let coords: Vec<&Coordinate> = path.coordinates.iter().collect();
        // we are doing the path forward and backward but technically we could check for the word backward
        // inside the find_word_count_in_path function
        total += find_word_count_in_path(matrix, &coords, &word_chars);
        total += find_word_count_in_path(matrix, &coords, &word_chars_reverse);
    }
    total
}

fn check_x_word_at_position(
    matrix: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    word_chars: &[char],
) -> i32 {
    // Check top left to bottom right diagonal
    // If false we can early exit
    if !(0..word_chars.len())
        .map(|i| word_chars[i] == matrix[row + i][col + i])
        .all(|x| x)
    {
        return 0;
    }

    // Check top right to bottom left diagonal
    // If true we have found the word in the two diagonals that come from the top corners
    if (0..word_chars.len())
        .map(|i| word_chars[i] == matrix[row + i][col + word_chars.len() - 1 - i])
        .all(|x| x)
    {
        return 1;
    }

    // Check bottom left to top right diagonal
    // If true we have found the word in the two diagonals that come from the left corners
    if (0..word_chars.len())
        .map(|i| word_chars[i] == matrix[row + word_chars.len() - 1 - i][col + i])
        .all(|x| x)
    {
        return 1;
    }
    0
}

fn find_x_word_count_in_matrix(matrix: &Vec<Vec<char>>, word: &str) -> i32 {
    let matrix_row_bound = matrix.len();
    let matrix_col_bound = matrix[0].len();

    let mut total = 0;
    let word_chars: Vec<char> = word.chars().collect();
    let word_chars_reverse = word_chars.iter().rev().copied().collect::<Vec<_>>();

    // we stop when we are on a row that is too close to the edge and cannot fit the word
    for row in 0..matrix_row_bound - word_chars.len() + 1 {
        // we stop when we are on a column that is too close to the edge and cannot fit the word
        for col in 0..matrix_col_bound - word_chars.len() + 1 {
            total += check_x_word_at_position(matrix, row, col, &word_chars);
            // handling diagonal from bottom corners and right corners by checking reversed word
            total += check_x_word_at_position(matrix, row, col, &word_chars_reverse);
        }
    }
    total
}

pub fn solve(file_path: &PathBuf) -> Result<()> {
    let input_matrix = read_input(file_path)?;
    let paths = generate_line_paths(input_matrix.len(), input_matrix[0].len());
    let word = "XMAS";
    let word_count = find_word_count_in_matrix(&input_matrix, paths, word);
    println!("The word count for the word `{}` is {}", word, word_count);

    let word: &str = "MAS";
    let x_word_count = find_x_word_count_in_matrix(&input_matrix, word);
    println!(
        "The X- word count for the word `{}` is {}",
        word, x_word_count
    );
    Ok(())
}
