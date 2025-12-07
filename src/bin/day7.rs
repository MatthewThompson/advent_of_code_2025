use std::fs;

const INPUT_PATH: &str = "inputs/7.txt";

/// https://adventofcode.com/2025/day/7
fn main() {
    let mut tachyon_manifold = match parse_input(INPUT_PATH) {
        Ok(input) => input,
        Err(e) => return println!("Failed with error: {}", e),
    };
    println!("Answer 1 is: {}", tachyon_manifold.sum_beam_splits());
    tachyon_manifold.clear_manifold();
    println!("Answer 2 is {}", tachyon_manifold.sum_beam_timelines());
}

/// Input should be a diagram of a manifold, with a start denoted as S at the top, followed by . for empty spaces
/// and ^ for beam splitters.
fn parse_input(input_path: &str) -> Result<TachyonManifold, String> {
    let input_text = fs::read_to_string(input_path).map_err(|e| e.to_string())?;
    let tiles = input_text
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'S' => Ok(Tile::Start),
                    '.' => Ok(Tile::Empty),
                    '^' => Ok(Tile::Splitter),
                    other => Err(format!("Found invalid tile char: {other}")),
                })
                .collect::<Result<Vec<Tile>, String>>()
        })
        .collect::<Result<Vec<Vec<Tile>>, String>>()?;
    Ok(TachyonManifold::new(tiles))
}

#[derive(Clone, Copy, Debug)]
enum Tile {
    Start,
    Empty,
    Beam(u64),
    Splitter,
}

#[derive(Clone, Debug)]
struct TachyonManifold {
    tiles: Vec<Vec<Tile>>,
    width: usize,
}

impl TachyonManifold {
    fn new(tiles: Vec<Vec<Tile>>) -> Self {
        let width = tiles[0].len();
        Self { tiles, width }
    }

    fn sum_beam_splits(&mut self) -> u64 {
        let mut times_split = 0;
        for i in 1..self.tiles.len() {
            for j in 0..self.width {
                if let Tile::Start | Tile::Beam(_) = self.tiles[i - 1][j] {
                    match self.tiles[i][j] {
                        Tile::Splitter => {
                            times_split += 1;
                            if j > 0 {
                                self.tiles[i][j - 1] = Tile::Beam(0);
                            }
                            if j < self.width - 1 {
                                self.tiles[i][j + 1] = Tile::Beam(0);
                            }
                        }
                        _ => {
                            self.tiles[i][j] = Tile::Beam(0);
                        }
                    }
                }
            }
        }
        times_split
    }

    fn add_beam_timelines(&mut self, i: usize, j: usize, timelines: u64) {
        match self.tiles[i][j] {
            Tile::Start => {}
            Tile::Empty => self.tiles[i][j] = Tile::Beam(timelines),
            Tile::Beam(n) => self.tiles[i][j] = Tile::Beam(n + timelines),
            Tile::Splitter => {}
        }
    }

    fn sum_beam_timelines(&mut self) -> u64 {
        for i in 1..self.tiles.len() {
            for j in 0..self.width {
                match self.tiles[i - 1][j] {
                    Tile::Start => {
                        self.add_beam_timelines(i, j, 1);
                    }
                    Tile::Beam(timelines) => match self.tiles[i][j] {
                        Tile::Splitter => {
                            if j > 0 {
                                self.add_beam_timelines(i, j - 1, timelines);
                            }
                            if j < self.width - 1 {
                                self.add_beam_timelines(i, j + 1, timelines);
                            }
                        }
                        Tile::Start => {}
                        Tile::Empty => {
                            self.add_beam_timelines(i, j, timelines);
                        }
                        Tile::Beam(_) => {
                            self.add_beam_timelines(i, j, timelines);
                        }
                    },
                    Tile::Empty => {}
                    Tile::Splitter => {}
                }
            }
        }

        let mut total_timelines = 0;
        self.tiles.last().unwrap().iter().for_each(|tile| {
            if let Tile::Beam(timelines) = tile {
                total_timelines += timelines;
            }
        });
        total_timelines
    }

    fn clear_manifold(&mut self) {
        for i in 1..self.tiles.len() {
            for j in 0..self.width {
                if let Tile::Beam(_) = self.tiles[i][j] {
                    self.tiles[i][j] = Tile::Empty;
                }
            }
        }
    }
}
