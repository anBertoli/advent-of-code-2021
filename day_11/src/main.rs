use colored::Colorize;
use day_11::octo::*;
use std::fs;

fn main() {
    println!(
        "Day 11! Using some {} and more {}!",
        "crates".red().bold(),
        "experiments".blue().bold().italic()
    );

    let numbers = read_input("input.txt");
    let octopuses = Octopuses::from_nums(&numbers).unwrap();
    let flashes = octopuses.clone().do_cycles(100);
    println!("Part 1, grid after 100 steps: \n{}", octopuses);
    println!("Number of flashes: {}", flashes);

    let cycles = octopuses.clone().until_all_flashes();
    println!("Part 2: number of cycles: {}", cycles);
}

fn read_input(path: &str) -> Vec<u8> {
    let file_contents = fs::read_to_string(path).unwrap();
    file_contents
        .lines()
        .flat_map(&str::chars)
        .map(|x: char| x.to_digit(10).unwrap() as u8)
        .collect()
}
