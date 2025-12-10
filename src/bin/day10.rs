use std::{collections::HashSet, fs};

const INPUT_PATH: &str = "inputs/10.txt";

/// https://adventofcode.com/2025/day/10
fn main() {
    let configs = match parse_input(INPUT_PATH) {
        Ok(input) => input,
        Err(e) => return println!("Failed with error: {}", e),
    };
    // Expected 385
    match sum_minimum_set_merges(&configs) {
        Ok(answer) => println!("Answer 1 is: {}", answer),
        Err(err) => println!("Failed with: {}", err),
    }
}

/// Input is expected to be a list of configurations, one config per line.
/// An example config would be [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
/// The initial square brackets indicate the target state. Where . is off and # is on.
/// The start state is all off.
/// The following numbers in parenthesis represent buttons, pressing one will toggle
/// the state at the indices listed in the button.
fn parse_input(input_path: &str) -> Result<Vec<Configuration>, String> {
    let input_text = fs::read_to_string(input_path).map_err(|e| e.to_string())?;
    input_text
        .lines()
        .map(|l| {
            let mut parts = l.split_whitespace();
            // TODO parse this properly
            let target = parts.next().unwrap();
            let buttons: Result<Vec<HashSet<u64>>, String> =
                parts.rev().skip(1).map(parse_button).collect();
            Ok(Configuration {
                target: parse_target(target)?,
                buttons: buttons?,
            })
        })
        .collect()
}

fn parse_target(string: &str) -> Result<HashSet<u64>, String> {
    let mut target_set = HashSet::new();
    let mut index = 0;
    for char in string.chars() {
        // TODO check properly formed with []
        match char {
            '.' => {
                index += 1;
            }
            '#' => {
                target_set.insert(index);
                index += 1;
            }
            _ => {}
        }
    }
    Ok(target_set)
}

fn parse_button(string: &str) -> Result<HashSet<u64>, String> {
    let mut button_indices = HashSet::new();
    for char in string.chars() {
        // TODO check properly formed with []
        match char {
            digit if digit.is_ascii_digit() => {
                let index = digit.to_digit(10).ok_or("Failed to parse digit")?;
                button_indices.insert(index as u64);
            }
            _ => {}
        }
    }
    Ok(button_indices)
}

fn sum_minimum_set_merges(configs: &[Configuration]) -> Result<u64, String> {
    let results: Vec<u64> = configs
        .iter()
        .map(find_minimum_button_presses_to_target)
        .collect::<Result<Vec<u64>, String>>()?;
    Ok(results.iter().sum())
}

fn find_minimum_button_presses_to_target(config: &Configuration) -> Result<u64, String> {
    for merge_size in 1..=config.buttons.len() {
        let merge_sets = all_combinations_for_size(&config.buttons, merge_size);
        if merge_sets.iter().any(|s| s.set_equals(&config.target)) {
            return Ok(merge_size as u64);
        }
    }
    Err(format!(
        "Failed to find working set for config with target {:?}",
        config.target
    ))
}

fn all_combinations_for_size(buttons: &[HashSet<u64>], target_size: usize) -> Vec<MergeSet> {
    let mut merge_sets = vec![];
    for i in 0..=(buttons.len() - target_size) {
        let starting_set = MergeSet::new(buttons[i].clone());
        merge_sets.extend(recursive_merge_from(
            starting_set,
            buttons,
            i + 1,
            target_size,
        ))
    }
    merge_sets
}

fn recursive_merge_from(
    starting_set: MergeSet,
    buttons: &[HashSet<u64>],
    start_index: usize,
    target_size: usize,
) -> Vec<MergeSet> {
    if starting_set.size >= target_size {
        return vec![starting_set];
    }
    let left_to_merge = target_size - starting_set.size;
    let mut merge_sets = vec![];
    for i in start_index..=(buttons.len() - left_to_merge) {
        merge_sets.extend(recursive_merge_from(
            starting_set.merge(&buttons[i]),
            buttons,
            i + 1,
            target_size,
        ));
    }
    merge_sets
}

struct Configuration {
    target: HashSet<u64>,
    buttons: Vec<HashSet<u64>>,
    // Remaining set for part 2
}

// A small wrapper around a set to keep track of how many sets it has been merged with.
// Merging in this context is the symmetric difference between two sets, that is merging
// two sets will return a set that has all the elements in one or the other but not both.
struct MergeSet {
    set: HashSet<u64>,
    size: usize,
}

impl MergeSet {
    fn new(set: HashSet<u64>) -> Self {
        Self { set, size: 1 }
    }

    fn merge(&self, other: &HashSet<u64>) -> MergeSet {
        let merged = self.set.symmetric_difference(other).copied().collect();
        MergeSet {
            set: merged,
            size: self.size + 1,
        }
    }

    fn set_equals(&self, set: &HashSet<u64>) -> bool {
        self.set == *set
    }
}
