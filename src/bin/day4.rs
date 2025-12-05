use std::fs;

const INPUT_PATH: &str = "inputs/4.txt";

/// https://adventofcode.com/2025/day/4
fn main() {
    let mut input = match parse_input(INPUT_PATH) {
        Ok(input) => input,
        Err(e) => return println!("Failed with error: {}", e),
    };
    println!("Answer 1 is: {}", count_accessible_rolls(&input));
    println!(
        "Answer 2 is: {}",
        count_accessible_rolls_with_recursive_removal(&mut input)
    );
}

#[derive(Clone, Copy, Debug)]
enum Tile {
    Empty,
    Paper,
}

struct Grid {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(tiles: Vec<Vec<Tile>>) -> Self {
        let width = tiles[0].len();
        let height = tiles.len();
        Self {
            tiles,
            width,
            height,
        }
    }

    fn is_tile_accessible(&self, tile_row: isize, tile_column: isize) -> bool {
        let mut adjacent_paper = 0;
        for row in -1..=1 {
            for col in -1..=1 {
                if row == 0 && col == 0 {
                    continue;
                }
                let neighbour_row = tile_row + row;
                let neighbour_col = tile_column + col;
                if neighbour_col as usize >= self.width
                    || neighbour_col < 0
                    || neighbour_row as usize >= self.height
                    || neighbour_row < 0
                {
                    continue;
                }
                if let Tile::Paper =
                    self.tiles[(tile_row + row) as usize][(tile_column + col) as usize]
                {
                    adjacent_paper += 1;
                }
            }
        }
        adjacent_paper < 4
    }

    fn remove_paper(&mut self, tile_row: usize, tile_column: usize) {
        self.tiles[tile_row][tile_column] = Tile::Empty;
    }

    fn count_and_remove_accessible_rolls(&mut self) -> u64 {
        let mut accessible = 0;
        for row in 0..self.height {
            for col in 0..self.width {
                if let Tile::Paper = self.tiles[row][col]
                    && self.is_tile_accessible(row as isize, col as isize) {
                        accessible += 1;
                        self.remove_paper(row, col);
                    }
            }
        }
        accessible
    }
}

/// The input is a series of lines, containing a list of . (empty) or @ (roll of paper).
/// We want to convert this into a list of list of numbers.
fn parse_input(input_path: &str) -> Result<Grid, String> {
    let input_text = fs::read_to_string(input_path).map_err(|e| e.to_string())?;
    let tiles = input_text
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Ok(Tile::Empty),
                    '@' => Ok(Tile::Paper),
                    other => Err(format!("Found invalid tile char: {other}")),
                })
                .collect::<Result<Vec<Tile>, String>>()
        })
        .collect::<Result<Vec<Vec<Tile>>, String>>()?;
    Ok(Grid::new(tiles))
}

/// Find all the paper rolls in the grid which are accessible. A paper roll is defined as
/// accessible if there are less than 4 rolls of paper adjacent to it, including diagonally
/// adjacent.
fn count_accessible_rolls(grid: &Grid) -> u64 {
    let mut accessible = 0;
    grid.tiles.iter().enumerate().for_each(|(row_number, row)| {
        row.iter().enumerate().for_each(|(column_number, tile)| {
            if let Tile::Paper = tile
                && grid.is_tile_accessible(row_number as isize, column_number as isize) {
                    accessible += 1;
                }
        });
    });
    accessible
}

fn count_accessible_rolls_with_recursive_removal(grid: &mut Grid) -> u64 {
    let mut removed = grid.count_and_remove_accessible_rolls();
    let mut accessible = removed;
    while removed > 0 {
        removed = grid.count_and_remove_accessible_rolls();
        accessible += removed;
    }
    accessible
}
