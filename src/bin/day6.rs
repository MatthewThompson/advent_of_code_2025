use std::fs;

const INPUT_PATH: &str = "inputs/6.txt";

/// https://adventofcode.com/2025/day/6
fn main() {
    let row_equations = match parse_input(INPUT_PATH) {
        Ok(input) => input,
        Err(e) => return println!("Failed with error: {}", e),
    };
    println!("Answer 1 is: {}", sum_equations(&row_equations));

    let column_equations = match parse_input_columns(INPUT_PATH) {
        Ok(input) => input,
        Err(e) => return println!("Failed with error: {}", e),
    };
    println!("Answer 2 is: {}", sum_equations(&column_equations));
}

/// Expecting a number of rows with an equal list of numbers followed by an equal list of operations.
/// Each column represents an equation.
fn parse_input(input_path: &str) -> Result<Vec<Equation>, String> {
    let input_text = fs::read_to_string(input_path).map_err(|e| e.to_string())?;
    let (value_rows, operations) = parse_values_and_operations(&input_text)?;
    let mut equations = vec![];
    for i in 0..operations.len() {
        let values: Vec<u64> = value_rows.iter().map(|row| row[i]).collect();
        equations.push(Equation {
            operation: operations[i],
            values,
        });
    }
    Ok(equations)
}
fn parse_values_and_operations(
    input_text: &str,
) -> Result<(Vec<Vec<u64>>, Vec<Operation>), String> {
    let mut value_rows = vec![];
    for line in input_text.lines() {
        match line.chars().peekable().peek() {
            Some('*') | Some('+') => return Ok((value_rows, parse_operation_row(line)?)),
            Some(_) => value_rows.push(parse_value_row(line)?),
            None => return Err("Unexpected empty line found".to_owned()),
        }
    }
    Err("No operations row found".to_owned())
}

fn parse_value_row(row: &str) -> Result<Vec<u64>, String> {
    row.split_whitespace()
        .map(|s| s.parse::<u64>().map_err(|e| e.to_string()))
        .collect()
}

fn parse_operation_row(row: &str) -> Result<Vec<Operation>, String> {
    row.split_whitespace()
        .map(|s| match s {
            "*" => Ok(Operation::Multiply),
            "+" => Ok(Operation::Plus),
            unexpected => Err(format!("Expected operation but found {unexpected}")),
        })
        .collect()
}

/// ...
fn parse_input_columns(input_path: &str) -> Result<Vec<Equation>, String> {
    let input_text = fs::read_to_string(input_path).map_err(|e| e.to_string())?;
    let mut equations = vec![];

    let char_rows: Vec<Vec<char>> = input_text.lines().map(|l| l.chars().collect()).collect();

    let mut values: Vec<u64> = vec![];
    let mut current_operation = Operation::Plus;
    for column in 0..char_rows[0].len() {
        let mut column_string = String::new();
        for char_row in &char_rows {
            match char_row[column] {
                '*' => current_operation = Operation::Multiply,
                '+' => current_operation = Operation::Plus,
                ' ' => {}
                digit => column_string.push(digit),
            }
        }
        let trimmed = column_string.trim();
        if trimmed.is_empty() {
            equations.push(Equation {
                operation: current_operation,
                values: std::mem::take(&mut values),
            });
            values.clear();
        } else {
            values.push(trimmed.parse::<u64>().map_err(|e| e.to_string())?);
        }
    }
    // No final empty column at the end
    equations.push(Equation {
        operation: current_operation,
        values: std::mem::take(&mut values),
    });

    Ok(equations)
}

#[derive(Copy, Clone, Debug)]
enum Operation {
    Plus,
    Multiply,
}

impl Operation {
    fn apply(&self, left: u64, right: u64) -> u64 {
        match &self {
            Operation::Plus => left + right,
            Operation::Multiply => left * right,
        }
    }
}

#[derive(Clone, Debug)]
struct Equation {
    operation: Operation,
    values: Vec<u64>,
}

impl Equation {
    fn calculate(&self) -> u64 {
        let mut value_iter = self.values.iter();
        match value_iter.next() {
            Some(initial) => {
                let mut total = *initial;
                value_iter.for_each(|value| {
                    total = self.operation.apply(total, *value);
                });
                total
            }
            None => 0,
        }
    }
}

fn sum_equations(equations: &[Equation]) -> u64 {
    equations.iter().map(|e| e.calculate()).sum()
}
