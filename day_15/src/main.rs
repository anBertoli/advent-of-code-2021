use day_15::*;
use std::fs;

fn main() {
    println!("Day 15!");

    let mut graph = parse_input("input.txt");
    let mut labels = Labels::new_from_graph(&graph);
    let (shortest_path, min_cost) = find_shortest_path(&graph, &mut labels);
    graph.print(&shortest_path);
    println!("Part 1, min cost: {}", min_cost);

    graph.enlarge();

    let mut labels = Labels::new_from_graph(&graph);
    let (_, min_cost) = find_shortest_path(&graph, &mut labels);
    println!("Part 2, min cost: {}", min_cost);
}

fn find_shortest_path(graph: &Graph, labels: &mut Labels) -> (Vec<(usize, usize)>, u32) {
    loop {
        extend_frontier(graph, labels);
        if labels.is_frontier_ended() {
            break;
        }
    }
    let (rows, cols) = graph.len();
    labels.get_shortest_path((rows - 1, cols - 1))
}

fn extend_frontier(graph: &Graph, labels: &mut Labels) {
    // Find nearest node among frontier ones. This will become
    // a stable label and is removed from th frontier ones.
    let (nearest_pos, nearest_label) = labels.find_nearest_in_frontier().unwrap();
    let nearest_dist = nearest_label.distance;
    labels.move_to_stable(nearest_pos);

    // Find the nodes near to the node currently inserted
    // into the stable ones and removed from the frontier.
    let neighbors: Vec<(usize, usize, u32)> = graph
        .neighbors(nearest_pos)
        .into_iter()
        .flatten()
        .filter(|&label_key| !labels.is_in_stable(label_key))
        .map(|label_key| (label_key.0, label_key.1, graph[label_key]))
        .collect();

    // Update each node adjacent to the newly stabilized node. If
    // the neighbor wasn't in the frontier, insert it. Otherwise,
    // check if the new distance for the node is better (minor)
    // than the current one. If yes, update the label.
    for neighbor in neighbors {
        let node_key = (neighbor.0, neighbor.1);
        match labels.get_from_frontier(node_key) {
            Some(label) => {
                if label.distance > nearest_dist + neighbor.2 {
                    label.distance = nearest_dist + neighbor.2;
                    label.next = nearest_pos;
                }
            }
            None => labels.insert_in_frontier(
                node_key,
                Label {
                    distance: nearest_dist + neighbor.2,
                    next: nearest_pos,
                },
            ),
        }
    }
}

fn parse_input(path: &str) -> Graph {
    let file_contents = fs::read_to_string(path).unwrap();
    Graph::from_lines(file_contents.lines())
}
