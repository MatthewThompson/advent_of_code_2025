use std::fs;

const INPUT_PATH: &str = "inputs/11.txt";

/// https://adventofcode.com/2025/day/11
fn main() {
    let configs = match parse_input(INPUT_PATH) {
        Ok(input) => input,
        Err(e) => return println!("Failed with error: {}", e),
    };
    todo!("Add answer 1");
}

/// TODO comment explaining format
fn parse_input(input_path: &str) -> Result<(), String> {
    let input_text = fs::read_to_string(input_path).map_err(|e| e.to_string())?;
    todo!("Parse input as correct type");
}
