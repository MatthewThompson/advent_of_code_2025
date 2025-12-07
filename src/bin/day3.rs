use std::fs;

const INPUT_PATH: &str = "inputs/3.txt";

/// https://adventofcode.com/2025/day/3
fn main() {
    let input = match parse_input(INPUT_PATH) {
        Ok(input) => input,
        Err(e) => return println!("Failed with error: {}", e),
    };
    println!("Answer 1 is: {}", sum_highest_2_digit_joltages(&input));
    println!("Answer 2 is: {}", sum_highest_12_digit_joltages(&input));
}

/// The input is a series of lines, each being a list of digits.
/// We want to convert this into a list of list of numbers.
fn parse_input(input_path: &str) -> Result<Vec<Vec<u64>>, String> {
    let input_text = fs::read_to_string(input_path).map_err(|e| e.to_string())?;
    input_text
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| {
                    char::to_digit(c, 10)
                        .map(|d| d as u64)
                        .ok_or(format!("failed to convert char {c} to digit"))
                })
                .collect()
        })
        .collect()
}

fn get_non_last_index_of_largest_battery(battery_bank: &[u64]) -> usize {
    let mut highest = 0;
    let mut index_of_highest = 0;
    battery_bank
        .iter()
        .enumerate()
        .take(battery_bank.len() - 1)
        .for_each(|(index, &battery)| {
            if battery > highest {
                index_of_highest = index;
                highest = battery;
            }
        });
    index_of_highest
}

fn get_highest_2_digit_joltage(battery_bank: &[u64]) -> u64 {
    let first_battery_index = get_non_last_index_of_largest_battery(battery_bank);
    let first_battery = battery_bank[first_battery_index];
    let mut second_battery = 1;
    battery_bank
        .iter()
        .skip(first_battery_index + 1)
        .for_each(|&battery| {
            if battery > second_battery {
                second_battery = battery;
            }
        });
    // "concatenate" the two digits
    first_battery * 10 + second_battery
}

fn sum_highest_2_digit_joltages(battery_banks: &[Vec<u64>]) -> u64 {
    battery_banks
        .iter()
        .map(|bank| get_highest_2_digit_joltage(bank))
        .sum()
}

fn get_highest_battery_index_in_range(
    battery_bank: &[u64],
    from_start: usize,
    from_end: usize,
) -> usize {
    let mut highest = 0;
    let mut index_of_highest = 0;
    battery_bank
        .iter()
        .enumerate()
        .take(battery_bank.len() - from_end)
        .skip(from_start)
        .for_each(|(index, &battery)| {
            if battery > highest {
                index_of_highest = index;
                highest = battery;
            }
        });
    index_of_highest
}

fn get_highest_12_digit_joltage(battery_bank: &[u64]) -> u64 {
    let mut remaining: u32 = 11;
    let mut previous_battery_index = get_highest_battery_index_in_range(
        battery_bank,
        0,
        // We always want there to be enough digits left at the end to choose the rest of the batteries
        remaining as usize,
    );
    let mut total: u64 = battery_bank[previous_battery_index] * 10_u64.pow(remaining);
    while remaining > 0 {
        remaining -= 1;
        let next_battery_index = get_highest_battery_index_in_range(
            battery_bank,
            previous_battery_index + 1,
            // We always want there to be enough digits left at the end to choose the rest of the batteries
            remaining as usize,
        );
        total += battery_bank[next_battery_index] * 10_u64.pow(remaining);
        previous_battery_index = next_battery_index;
    }

    total
}

fn sum_highest_12_digit_joltages(battery_banks: &[Vec<u64>]) -> u64 {
    battery_banks
        .iter()
        .map(|bank| get_highest_12_digit_joltage(bank))
        .sum()
}
