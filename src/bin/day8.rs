use std::{
    collections::{BinaryHeap, HashSet},
    fs,
};

const INPUT_PATH: &str = "inputs/8.txt";

/// https://adventofcode.com/2025/day/8
fn main() {
    let coordinates = match parse_input(INPUT_PATH) {
        Ok(input) => input,
        Err(e) => return println!("Failed with error: {}", e),
    };
    let mut graph = Graph::new(&coordinates);
    println!("Answer 1 is: {}", graph.calculate_thing());
}

/// Expected input is a list of 3D coordinates.
fn parse_input(input_path: &str) -> Result<Vec<Point>, String> {
    let input_text = fs::read_to_string(input_path).map_err(|e| e.to_string())?;
    input_text
        .lines()
        .map(|l| {
            let parts: Vec<_> = l.split(",").collect();
            match parts.as_slice() {
                [x, y, z] => Ok(Point(
                    x.parse::<i64>().map_err(|e| e.to_string())?,
                    y.parse::<i64>().map_err(|e| e.to_string())?,
                    z.parse::<i64>().map_err(|e| e.to_string())?,
                )),
                _ => Err("Invalid coordinate row".to_owned()),
            }
        })
        .collect()
}

struct Graph {
    junction_boxes: Vec<JunctionBox>,
    pair_distances: BinaryHeap<Edge>,
    // Each circuit contains a set of points by index.
    circuits: Vec<HashSet<usize>>,
}

impl Graph {
    fn new(junction_box_locations: &[Point]) -> Self {
        let network_size = junction_box_locations.len();
        let mut pair_distances = BinaryHeap::new();
        for from in 0..network_size {
            for to in from + 1..network_size {
                let distance = junction_box_locations[from].distance(&junction_box_locations[to]);
                let edge = Edge { from, to, distance };
                pair_distances.push(edge);
            }
        }
        let junction_boxes = junction_box_locations
            .iter()
            .enumerate()
            .map(|(i, _)| JunctionBox { circuit: i })
            .collect();
        let circuits = junction_box_locations
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let mut circuit = HashSet::new();
                circuit.insert(i);
                circuit
            })
            .collect();
        Self {
            junction_boxes,
            pair_distances,
            circuits,
        }
    }

    fn calculate_thing(&mut self) -> usize {
        for _i in 0..1000 {
            let next_shortest_edge = self.pair_distances.pop().unwrap();
            let from_box_circuit = self.junction_boxes[next_shortest_edge.from].circuit;
            let to_box_circuit = self.junction_boxes[next_shortest_edge.to].circuit;

            // Take out the from circuit so we can merge it with the other one
            let from_circuit = std::mem::take(&mut self.circuits[from_box_circuit]);
            for &thingy in &from_circuit {
                // Change the location of the boxes we're moving.
                self.junction_boxes[thingy].circuit = to_box_circuit;
            }
            self.circuits[to_box_circuit].extend(from_circuit);
        }
        self.circuits.sort_by_key(|a| a.len());
        let mut top_three = self.circuits.iter().rev();
        top_three.next().unwrap().len()
            * top_three.next().unwrap().len()
            * top_three.next().unwrap().len()
    }
}

struct JunctionBox {
    // Needed for part 2
    // location: Point,
    // Maintain which circuit this is in.
    circuit: usize,
}

#[derive(Debug, Copy, Clone, Hash)]
struct Point(i64, i64, i64);

#[derive(Debug, Copy, Clone, PartialEq)]
struct Edge {
    from: usize,
    to: usize,
    distance: f64,
}

impl Eq for Edge {}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.total_cmp(&self.distance)
    }
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        let squares = ((self.0 - other.0).pow(2)
            + (self.1 - other.1).pow(2)
            + (self.2 - other.2).pow(2)) as f64;
        squares.sqrt()
    }
}
