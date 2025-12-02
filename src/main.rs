use std::fs;

const INPUT_PATH: &str = "src/input.txt";
const START_POSITION: i32 = 50;
const DIAL_SIZE: i32 = 100;

/// Input is a list of directions to turn in the format [LR](\d+)
/// e.g.
/// L23
/// R234
/// R43
/// 
/// We parse this and return a vector of integers, with positive values being a turn to the right (clockwise)
/// and negative values to the left.
fn parse_input(input_path: &str) -> Result<Vec<i32>, String> {
    let input_text = fs::read_to_string(input_path).map_err(|e| e.to_string())?;
    input_text
        .lines()
        .map(|l| {
            let (dir, degrees_str) = l.split_at(1);
            let degrees = degrees_str.parse::<i32>().map_err(|e| e.to_string())?;
            match dir {
                "L" => Ok(-degrees),
                "R" => Ok(degrees),
                s => Err(format!("Unexpected start of line: {}", s)),
            }
        })
        .collect()
}

fn calculate_times_landed_on_zero(turns: &[i32]) -> u32 {
    let mut position = START_POSITION;
    let mut reached_zero = 0;
    turns.iter().for_each(|degrees| {
        position = i32::rem_euclid(position + degrees, DIAL_SIZE);
        if position == 0 {
            reached_zero += 1;
        }
    });
    reached_zero
}

fn calculate_times_passed_zero(turns: &[i32]) -> u32 {
    let mut position = START_POSITION;
    let mut passed_zero: u32 = 0;
    turns.iter().for_each(|&degrees| {
        let distance_to_zero = match degrees {
            d if d < 0 => { if position == 0 { 100 } else { position } },
            d if d > 0 => DIAL_SIZE - position,
            _ => 100,
        };
        let positive_degrees = i32::abs(degrees);
        if positive_degrees >= distance_to_zero {
            passed_zero += 1;
            let remaining_degrees = positive_degrees - distance_to_zero;
            passed_zero += (remaining_degrees / DIAL_SIZE) as u32;
        }
        position = i32::rem_euclid(position + degrees, DIAL_SIZE);
    });
    passed_zero
}

fn main() {
    let input = match parse_input(INPUT_PATH) {
        Ok(input) => input,
        Err(e) => return println!("Failed with error: {}", e),
    };
    println!("Answer 1 is: {}", calculate_times_landed_on_zero(&input));
    println!("Answer 2 is: {}", calculate_times_passed_zero(&input));
}
