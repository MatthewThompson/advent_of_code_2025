use std::{cmp, fs, ops::RangeInclusive};

const INPUT_PATH: &str = "inputs/5.txt";

/// https://adventofcode.com/2025/day/5
fn main() {
    let (ranges, ids) = match parse_input(INPUT_PATH) {
        Ok(input) => input,
        Err(e) => return println!("Failed with error: {}", e),
    };
    println!("Answer 1 is: {}", count_ids_in_ranges(&ranges, &ids));
    println!("Answer 2 is: {}", count_overlapping_ranges_size(ranges));
}

/// Expecting a list of ranges in the form number-number followed by an empty line
/// followed by a list of numbers.
fn parse_input(input_path: &str) -> Result<(Vec<RangeInclusive<u64>>, Vec<u64>), String> {
    let input_text = fs::read_to_string(input_path).map_err(|e| e.to_string())?;
    // Windows only, there's probably an easy way to make it platform independent.
    let parts: Vec<&str> = input_text.split("\r\n\r\n").collect();
    match parts.as_slice() {
        [ranges, ids] => {
            let ranges = parse_ranges(ranges)?;
            let ids = parse_ids(ids)?;
            Ok((ranges, ids))
        }
        _ => Err("Invalid number of parts".to_owned())
    }
}

/// Input expected to be a list of new line separated ranges, two positive integers separated by a dash.
fn parse_ranges(input: &str) -> Result<Vec<RangeInclusive<u64>>, String> {
    input
        .lines()
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
                Ok(RangeInclusive::new(id1, id2))
            })?
        })
        .collect()
}

/// Input expected to be a list of new line separated positive integers.
fn parse_ids(input: &str) -> Result<Vec<u64>, String> {
    input
        .lines()
        .map(|l| l.parse::<u64>().map_err(|e| e.to_string()))
        .collect()
}

/// Returns the number of ids that are in the ranges.
fn count_ids_in_ranges(ranges: &[RangeInclusive<u64>], ids: &[u64]) -> u64 {
    let mut ids_in_range = 0;
    ids.iter().for_each(|id| {
        if ranges.iter().any(|range| range.contains(id)) {
            ids_in_range += 1;
        }
    });
    ids_in_range
}

/// Returns the total number of numbers in all the ranges provided. Will not double count a number
/// if it is in multiple ranges.
fn count_overlapping_ranges_size(mut ranges: Vec<RangeInclusive<u64>>) -> u64 {
    // Sort the potentially overlapping ranges by their start value ascending.
    ranges.sort_by(|r, r2| r.start().cmp(r2.start()));

    let mut non_overlapping_ranges = vec![];

    let mut ranges = ranges.iter();
    let mut current_range = ranges.next();
    // For each "starting" range, we want to merge all overlapping ranges. Once we find the first range that has no overlap
    // we add our current merged range to a new vector and start over.
    // By converting this list of overlapping ranges into a shorter list of non overlapping ranges. We can simply count each
    // range individually to get the overall size of the ranges.
    while let Some(range) = current_range {
        let range_start = range.start();
        let mut range_end = range.end();

        let mut maybe_next_range = ranges.next();
        while let Some(next_range) = maybe_next_range
            // Non overlapping, add current range to list and continue
            && next_range.start() <= range_end
        {
            // Merge this range into the current one by extending the range if this one has a larger end.
            range_end = cmp::max(range_end, next_range.end());
            maybe_next_range = ranges.next();
        }
        non_overlapping_ranges.push(RangeInclusive::new(*range_start, *range_end));
        // The next range is either none, or a non overlapping range. Either way we want to start the loop again with this range as our starting range.
        current_range = maybe_next_range;
    }
    // If we know none of the ranges overlap we can easily find their total size just by summing
    // the sizes of each range.
    count_non_overlapping_range_size(non_overlapping_ranges)
}

/// Sums the size of all given inclusive ranges.
fn count_non_overlapping_range_size(ranges: Vec<RangeInclusive<u64>>) -> u64 {
    ranges.iter().map(|range| range.end() + 1 - range.start()).sum()
}
