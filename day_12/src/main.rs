use day_12::*;
use std::fs;

fn main() {
    println!(
        "Day 12! I know, I know this is a mess. I was trying \
     to experiment with self referential collections and \
     unfortunately switched to RC."
    );

    let edges = read_input("input.txt");
    let mut graph = Graph::from_edges(edges);

    let paths = graph.find_all_paths_part_1();
    println!("Part 1: {}", paths.len());

    let paths = graph.find_all_paths_part_2();
    println!("Part 2: {}", paths.len())
}

fn read_input(path: &str) -> Vec<(String, String)> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .map(|(r, l)| (r.to_owned(), l.to_owned()))
        .collect()
}
