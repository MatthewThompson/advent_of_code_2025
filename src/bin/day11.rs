use std::{collections::HashMap, fs};

const INPUT_PATH: &str = "inputs/11.txt";

/// https://adventofcode.com/2025/day/11
fn main() {
    let edges = match parse_input(INPUT_PATH) {
        Ok(input) => input,
        Err(e) => return println!("Failed with error: {}", e),
    };
    println!("Answer 1 is {}", count_paths_start_to_end(&edges));
    println!("Answer 2 is {}", count_paths_sr_to_end_via_fft_dac(&edges));
}

/// Expecting a directed acyclic graph in format of a row containing a name of a node
/// followed by all the nodes it connects to. Some nodes have specific names:
///   - you -> Start
///   - out -> End
///   - svr -> ServerRack
///   - dac -> Dac
///   - fft -> Fft
///
/// The rest are short strings, since the value isn't something we care about we instead assign incrementing integer IDs to each to save on space and computation.
fn parse_input(input_path: &str) -> Result<HashMap<GraphNode, Vec<GraphNode>>, String> {
    let input_text = fs::read_to_string(input_path).map_err(|e| e.to_string())?;
    let mut name_id_map = NameIdentifierMap::new();
    let mut graph = HashMap::new();
    for line in input_text.lines() {
        let parts: Vec<_> = line.split(":").collect();
        match parts.as_slice() {
            [input, rest] => {
                let from = parse_node(input, &mut name_id_map);
                let to_nodes: Vec<_> = rest
                    .split_whitespace()
                    .map(|name| parse_node(name, &mut name_id_map))
                    .collect();
                graph.insert(from, to_nodes);
            }
            _ => return Err("Invalid input".to_owned()),
        }
    }
    Ok(graph)
}

/// Return a graph node for a string name. If it is a known keyword name we assign a special
/// graph node, otherwise we just return a [`Node`] with an incrementing ID.
fn parse_node(node_name: &str, name_id_map: &mut NameIdentifierMap) -> GraphNode {
    match node_name {
        "you" => GraphNode::Start,
        "out" => GraphNode::End,
        "svr" => GraphNode::ServerRack,
        "dac" => GraphNode::Dac,
        "fft" => GraphNode::Fft,
        name => GraphNode::Node(name_id_map.register_and_get_id(name)),
    }
}

struct NameIdentifierMap {
    node_name_to_id: HashMap<String, u64>,
    next_id: u64,
}

impl NameIdentifierMap {
    fn new() -> Self {
        Self {
            node_name_to_id: HashMap::new(),
            next_id: 0,
        }
    }

    fn register_and_get_id(&mut self, name: &str) -> u64 {
        match self.node_name_to_id.get(name) {
            Some(&existing_id) => existing_id,
            None => {
                let id = self.next_id;
                self.node_name_to_id.insert(name.to_string(), id);
                self.next_id += 1;
                id
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum GraphNode {
    Start,
    ServerRack,
    Dac,
    Fft,
    Node(u64),
    End,
}

fn count_paths_start_to_end(graph: &HashMap<GraphNode, Vec<GraphNode>>) -> u64 {
    let mut path_lengths = HashMap::new();
    count_paths_dfs(graph, GraphNode::Start, GraphNode::End, &mut path_lengths)
}

fn count_paths_sr_to_end_via_fft_dac(graph: &HashMap<GraphNode, Vec<GraphNode>>) -> u64 {
    let mut path_lengths = HashMap::new();
    let sr_to_fft = count_paths_dfs(
        graph,
        GraphNode::ServerRack,
        GraphNode::Fft,
        &mut path_lengths,
    );
    let mut path_lengths = HashMap::new();
    let fft_to_dac = count_paths_dfs(graph, GraphNode::Fft, GraphNode::Dac, &mut path_lengths);
    let mut path_lengths = HashMap::new();
    let dac_to_end = count_paths_dfs(graph, GraphNode::Dac, GraphNode::End, &mut path_lengths);
    sr_to_fft * fft_to_dac * dac_to_end
}

// Depth first search from start to end, maintaining a map of nodes and the number of paths from that node to the end.
fn count_paths_dfs(
    graph: &HashMap<GraphNode, Vec<GraphNode>>,
    start: GraphNode,
    end: GraphNode,
    // The distance taken to get from the overall start to each node
    paths_to_end_by_node: &mut HashMap<GraphNode, u64>,
) -> u64 {
    if start == end {
        return 1;
    }
    // We have already seen this node before in a different path, return the known number of paths from here to the end without calculating
    if let Some(&len) = paths_to_end_by_node.get(&start) {
        return len;
    }
    let children = vec![];
    let children = graph.get(&start).unwrap_or(&children);
    let total_paths_to_end = children
        .iter()
        .map(|child| count_paths_dfs(graph, *child, end, paths_to_end_by_node))
        .sum();
    paths_to_end_by_node
        .entry(start)
        .or_insert(total_paths_to_end);
    total_paths_to_end
}
