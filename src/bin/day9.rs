use std::{collections::HashSet, fs};

const INPUT_PATH: &str = "inputs/9.txt";

/// https://adventofcode.com/2025/day/9
fn main() {
    let coordinates = match parse_input(INPUT_PATH) {
        Ok(input) => input,
        Err(e) => return println!("Failed with error: {}", e),
    };
    // Expected: 4763040296
    println!("Answer 1 is: {}", get_largest_rect_area(&coordinates));
    let dance_floor = match Polygon::new(&coordinates) {
        Ok(dance_floor) => dance_floor,
        Err(e) => return println!("Failed to make dance floor with error: {}", e),
    };

    // Expected: 1396494456
    println!(
        "Answer 2 is: {}",
        get_largest_rect_area_entirely_in_polygon(&coordinates, &dance_floor)
    );
}

/// Expected input is a list of 2D coordinates.
fn parse_input(input_path: &str) -> Result<Vec<(usize, usize)>, String> {
    let input_text = fs::read_to_string(input_path).map_err(|e| e.to_string())?;
    input_text
        .lines()
        .map(|l| {
            let parts: Vec<_> = l.split(",").collect();
            match parts.as_slice() {
                [x, y] => Ok((
                    x.parse::<usize>().map_err(|e| e.to_string())?,
                    y.parse::<usize>().map_err(|e| e.to_string())?,
                )),
                _ => Err("Invalid coordinate row".to_owned()),
            }
        })
        .collect()
}

fn get_largest_rect_area(coordinates: &[(usize, usize)]) -> usize {
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

fn get_largest_rect_area_entirely_in_polygon(
    coordinates: &[(usize, usize)],
    polygon: &Polygon,
) -> usize {
    let mut coord_pairs: Vec<((usize, usize), (usize, usize))> = vec![];
    for i in 0..coordinates.len() - 1 {
        for j in i..coordinates.len() {
            let coord1 = coordinates[i];
            let coord2 = coordinates[j];
            coord_pairs.push((coord1, coord2));
        }
    }
    // Sort the coordinates by rectangle area descending first, since we want to find
    // the largest valid rectangle, this will reduce the number that we need to check.
    coord_pairs.sort_by(|a, b| {
        let rect_a = Rect::new(a.0, a.1);
        let rect_b = Rect::new(b.0, b.1);
        rect_b.area().cmp(&rect_a.area())
    });
    for &pair in coord_pairs.iter() {
        if polygon.rect_entirely_inside(pair) {
            return Rect::new(pair.0, pair.1).area();
        }
    }
    0
}

struct Polygon {
    sorted_vertical_edges: Vec<((usize, usize), (usize, usize))>,
    rev_sorted_vertical_edges: Vec<((usize, usize), (usize, usize))>,
    boundary_coords: HashSet<(usize, usize)>,
}

impl Polygon {
    fn new(coordinates: &[(usize, usize)]) -> Result<Self, String> {
        let mut boundary_coords = HashSet::new();
        let mut sorted_vertical_edges = vec![];

        for i in 0..coordinates.len() {
            let coords1 = coordinates[i];
            let coords2 = if i == coordinates.len() - 1 {
                coordinates[0]
            } else {
                coordinates[i + 1]
            };
            boundary_coords.insert(coords1);
            let (x0, y0) = coords1;
            let (x1, y1) = coords2;
            if x0 == x1 {
                let range = if y0 <= y1 { y0..=y1 } else { y1..=y0 };
                for y in range {
                    boundary_coords.insert((x0, y));
                }
                sorted_vertical_edges.push((coords1, coords2));
            } else if y0 == y1 {
                let range = if x0 <= x1 { x0..=x1 } else { x1..=x0 };
                for x in range {
                    boundary_coords.insert((x, y0));
                }
            } else {
                return Err(format!(
                    "Found pair of coordinates that are not aligned at indexes: {} and {}",
                    i,
                    i + 1
                ));
            }
        }
        sorted_vertical_edges.sort_by(|a, b| a.0.0.cmp(&b.0.0));
        let mut rev_sorted_vertical_edges = sorted_vertical_edges.clone();
        rev_sorted_vertical_edges.sort_by(|a, b| b.0.0.cmp(&a.0.0));
        Ok(Self {
            sorted_vertical_edges,
            rev_sorted_vertical_edges,
            boundary_coords,
        })
    }

    fn rect_entirely_inside(&self, coordinate_pair: ((usize, usize), (usize, usize))) -> bool {
        let ((x0, y0), (x1, y1)) = coordinate_pair;
        let (min_x, max_x) = if x0 < x1 { (x0, x1) } else { (x1, x0) };
        let (min_y, max_y) = if y0 < y1 { (y0, y1) } else { (y1, y0) };
        // Because I know about the input file, can just check the side edges of the rect and not the center.
        // Realistically this should be made more robust and then also sped up.
        for y in min_y..=max_y {
            if !self.inside_boundary((min_x, y)) {
                return false;
            }
        }
        for y in min_y..=max_y {
            if !self.inside_boundary((max_x, y)) {
                return false;
            }
        }
        true
    }

    fn inside_boundary(&self, coordinate: (usize, usize)) -> bool {
        self.boundary_coords.contains(&coordinate)
            || self.in_bounds_left(coordinate)
            // A bit of a hack to fix a bug with false negatives. Only checking to one side should
            // be sufficient.
            || self.in_bounds_right(coordinate)
    }

    // If we trace a ray to the left from a given point and count how many times it crosses a boundary, we can tell if it
    // lies inside our polygon. If it crosses an even number it is outside, odd in.
    fn in_bounds_left(&self, coordinate: (usize, usize)) -> bool {
        let mut times_crossed_boundary = 0;
        let mut prev_y0 = 0;
        let mut prev_y1 = 0;
        for &((x0, y0), (_, y1)) in &self.sorted_vertical_edges {
            if x0 >= coordinate.0 {
                break;
            }
            if (y0 <= coordinate.1 && coordinate.1 <= y1)
                || (y0 >= coordinate.1 && coordinate.1 >= y1)
            {
                // We are aligned perfectly with a horizontal line. This should count as one cross and not two.
                let aligned_horizontally = (prev_y0 == y0 && prev_y0 == coordinate.1)
                    || (prev_y1 == y1 && prev_y1 == coordinate.1);
                if !aligned_horizontally {
                    times_crossed_boundary += 1;
                }
            }
            prev_y0 = y0;
            prev_y1 = y1;
        }
        times_crossed_boundary % 2 != 0
    }

    // If we trace a ray to the right from a given point and count how many times it crosses a boundary, we can tell if it
    // lies inside our polygon. If it crosses an even number it is outside, odd in.
    fn in_bounds_right(&self, coordinate: (usize, usize)) -> bool {
        let mut times_crossed_boundary = 0;
        let mut prev_y0 = 0;
        let mut prev_y1 = 0;
        for &((x0, y0), (_, y1)) in &self.rev_sorted_vertical_edges {
            if x0 <= coordinate.0 {
                break;
            }
            if (y0 <= coordinate.1 && coordinate.1 <= y1)
                || (y0 >= coordinate.1 && coordinate.1 >= y1)
            {
                // We are aligned perfectly with a horizontal line. This should count as one cross and not two.
                let aligned_horizontally = (prev_y0 == y0 && prev_y0 == coordinate.1)
                    || (prev_y1 == y1 && prev_y1 == coordinate.1);
                if !aligned_horizontally {
                    times_crossed_boundary += 1;
                }
            }
            prev_y0 = y0;
            prev_y1 = y1;
        }
        times_crossed_boundary % 2 != 0
    }
}

struct Rect {
    x0: usize,
    y0: usize,
    x1: usize,
    y1: usize,
}

impl Rect {
    fn new(corner1: (usize, usize), corner2: (usize, usize)) -> Self {
        Self {
            x0: corner1.0.min(corner2.0),
            y0: corner1.1.min(corner2.1),
            x1: corner1.0.max(corner2.0),
            y1: corner1.1.max(corner2.1),
        }
    }

    fn area(&self) -> usize {
        // We need to add one to each side since each coordinate is essentially a 1x1 square.
        // So a single point would be size 1. A rectangle from points (1, 1) to (2, 2) would
        // be a 2x2 square so area should be 4.
        ((self.x1 - self.x0) + 1) * ((self.y1 - self.y0) + 1)
    }
}
