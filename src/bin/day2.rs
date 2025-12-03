use std::fs;

const INPUT_PATH: &str = "inputs/2.txt";

/// Input is a list of comma separated ranges in the format \d+-\d+
/// e.g. 11-22,95-115,998-1012,1188511880-1188511890
fn parse_input(input_path: &str) -> Result<Vec<(u64, u64)>, String> {
    let input_text = fs::read_to_string(input_path).map_err(|e| e.to_string())?;
    input_text
        .split(',')
        .map(|range| {
            let mut ids = range.split('-');
            if let (Some(id1), Some(id2), None) = (ids.next(), ids.next(), ids.next()) {
                return Ok((id1, id2));
            }
            Err("Invalid number of IDs found in range")
        })
        .map(|result| {
            result.map(|(id1_str, id2_str)| {
                let id1 = id1_str.parse::<u64>().map_err(|e| e.to_string())?;
                let id2 = id2_str.parse::<u64>().map_err(|e| e.to_string())?;
                Ok((id1, id2))
            })?
        })
        .collect()
}

/// An ID is considered to be invalid if it is the same digits repeated twice.
/// e.g. 11, 123123, 44004400
fn id_is_double_sequence(id: &u64) -> bool {
    // there's probably a nice mathsy way to do this??
    let id_str = id.to_string();
    let (left, right) = id_str.split_at(id_str.len() / 2);
    left == right
}

/// An ID can also be considered to be invalid if it is the same digits repeated any number of times.
/// e.g. 111, 121212, 111111, 456456
fn id_has_repeated_digit_sequence(id: &u64) -> bool {
    let id_str = id.to_string();
    let id_len = id_str.len();
    if id_len == 1 {
        return false;
    }
    for sub_sequence_len in 1..id_len {
        // Cannot be a repeated sequence if the length is not divisible by the sequence size.
        if id_len % sub_sequence_len != 0 {
            continue;
        }
        let mut chunks = id_str.as_bytes().chunks(sub_sequence_len);
        let first = chunks.next();
        if let Some(first) = first {
            if chunks.all(|c| c == first) {
                return true;
            }
        }
    }
    false
}

fn invalid_ids_in_range(range: &(u64, u64), is_invalid: &dyn Fn(&u64) -> bool) -> Vec<u64> {
    (range.0..=range.1).filter(is_invalid).collect()
}

fn sum_invalid_ids_in_ranges(ranges: &[(u64, u64)], is_invalid: &dyn Fn(&u64) -> bool) -> u64 {
    ranges
        .iter()
        .flat_map(|range| invalid_ids_in_range(range, is_invalid))
        .sum()
}

fn main() {
    let input = match parse_input(INPUT_PATH) {
        Ok(input) => input,
        Err(e) => return println!("Failed with error: {}", e),
    };
    println!(
        "Answer 1 is: {}",
        sum_invalid_ids_in_ranges(&input, &id_is_double_sequence)
    );
    println!(
        "Answer 2 is: {}",
        sum_invalid_ids_in_ranges(&input, &id_has_repeated_digit_sequence)
    );
}
