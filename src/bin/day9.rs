use std::fs;

const INPUT_PATH: &str = "inputs/9.txt";

/// https://adventofcode.com/2025/day/9
fn main() {
    let coordinates = match parse_input(INPUT_PATH) {
        Ok(input) => input,
        Err(e) => return println!("Failed with error: {}", e),
    };
    println!("Answer 1 is: {}", get_largest_rect_area(&coordinates));
}

/// Expected input is a list of 2D coordinates.
fn parse_input(input_path: &str) -> Result<Vec<(i64, i64)>, String> {
    let input_text = fs::read_to_string(input_path).map_err(|e| e.to_string())?;
    input_text
        .lines()
        .map(|l| {
            let parts: Vec<_> = l.split(",").collect();
            match parts.as_slice() {
                [x, y] => Ok((
                    x.parse::<i64>().map_err(|e| e.to_string())?,
                    y.parse::<i64>().map_err(|e| e.to_string())?,
                )),
                _ => Err("Invalid coordinate row".to_owned()),
            }
        })
        .collect()
}

fn get_largest_rect_area(coordinates: &[(i64, i64)]) -> i64 {
    let mut largest_area = 0;
    for i in 0..coordinates.len() - 1 {
        for j in i..coordinates.len() {
            let area = Rect::new(coordinates[i], coordinates[j]).area();
            if area > largest_area {
                largest_area = area;
            }
        }
    }
    largest_area
}

struct Rect {
    corner1: (i64, i64),
    corner2: (i64, i64),
}

impl Rect {
    fn new(corner1: (i64, i64), corner2: (i64, i64)) -> Self {
        Self { corner1, corner2 }
    }

    fn area(&self) -> i64 {
        (i64::abs(self.corner1.0 - self.corner2.0) + 1)
            * (i64::abs(self.corner1.1 - self.corner2.1) + 1)
    }
}
