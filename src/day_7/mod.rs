use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Multiply,
    Concat
}

impl Operation {
    fn compute(&self, a: i64, b: i64) -> i64 {
        match self {
            Self::Add => a + b,
            Self::Multiply => a * b,
            // We are doing an unwrap here because we control a and b 
            Self::Concat => {(a.to_string() + b.to_string().as_str()).parse::<i64>().unwrap()}
        }
    }
}

fn all_combinations(values: &[Operation], n: usize) -> Vec<Vec<Operation>> {
    if n == 0 {
        return vec![vec![]];
    }

    let mut result = Vec::new();
    // First, recursively get all combinations of length (n - 1)
    let shorter = all_combinations(values, n - 1);

    // For each combination of length (n - 1), append each possible value to create length-n combos
    for combo in shorter {
        for &v in values {
            let mut new_combo = combo.clone();
            new_combo.push(v);
            result.push(new_combo);
        }
    }
    result
}

#[derive(Debug)]
struct Equation {
    output: i64,
    inputs: Vec<i64>,
}

impl Equation {
    fn compute_valid_solution(&self, all_ops: &[Operation]) -> i64 {
        for ops in all_combinations(all_ops, self.inputs.len() - 1)
        {
            if self.is_valid_solution(&ops) {
                return self.output;
            }
        }
        0
    }

    fn is_valid_solution(&self, ops: &[Operation]) -> bool {
        if ops.len() != self.inputs.len() - 1 {
            return false;
        }

        self.inputs
            .iter()
            .skip(1)
            .zip(ops)
            .fold(self.inputs[0], |acc, (&val, &op)| op.compute(acc, val))
            == self.output
    }
}

fn read_input(file_path: &PathBuf) -> Result<Vec<Equation>> {
    let input = fs::read_to_string(file_path)?;
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (output, inputs) = line
                .split_once(':')
                .context(format!("Invalid line format, missing ':', `{}`", line))?;
            Ok(Equation {
                output: output.trim().parse().context(format!("Failed to parse output for line: `{}`", line))?,
                inputs: inputs
                    .trim_start()
                    .split(' ')
                    .map(|n| n.trim().parse())
                    .collect::<std::result::Result<_, _>>()
                    .context(format!("Failed to parse inputs: `{}`", inputs))?,
            })
        })
        .collect()
}

pub fn solve(file_path: &PathBuf) -> Result<()> {
    let mut total = 0;
    let equations = read_input(file_path)?;
    // part 1:
    for eq in equations.iter() {
        total += eq.compute_valid_solution(&[Operation::Add, Operation::Multiply]);
    }
    println!("the total value is {}", total);
    let mut total = 0;

    // part 2:
    for eq in equations.iter() {
        total += eq.compute_valid_solution(&[Operation::Add, Operation::Multiply, Operation::Concat]);
    }
    println!("the total value when including concat operator is {}", total);

    Ok(())
}
