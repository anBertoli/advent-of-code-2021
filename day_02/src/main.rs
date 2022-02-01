use crate::Command::{Down, Forward, Up};
use std::fs;

fn main() {
    println!("Day 2!");

    let commands = read_input("input.txt").expect("invalid text");

    let (horizontal, depth) = calc_position_part_1(&commands);
    println!(
        "part 1, horizontal {}, depth {}, product {}",
        horizontal,
        depth,
        horizontal * depth,
    );

    let (horizontal, depth, aim) = calc_position_part_2(&commands);
    println!(
        "part 2, horizontal {}, depth {}, aim {}, product {}",
        horizontal,
        depth,
        aim,
        horizontal * depth,
    );
}

fn calc_position_part_1(commands: &[Command]) -> (u32, u32) {
    let mut pos: (u32, u32) = (0, 0);
    for cmd in commands {
        match cmd {
            Forward(n) => pos.0 += *n as u32,
            Up(n) => pos.1 -= *n as u32,
            Down(n) => pos.1 += *n as u32,
        };
    }
    pos
}

fn calc_position_part_2(commands: &[Command]) -> (u32, u32, u32) {
    let mut pos: (u32, u32, u32) = (0, 0, 0);
    for cmd in commands {
        match cmd {
            Forward(n) => {
                pos.0 += *n as u32;
                pos.1 += pos.2 * (*n as u32);
            }
            Up(n) => pos.2 -= *n as u32,
            Down(n) => pos.2 += *n as u32,
        };
    }
    pos
}

fn read_input(path: &str) -> Result<Vec<Command>, String> {
    let file = match fs::read_to_string(path) {
        Err(err) => return Err(err.to_string()),
        Ok(f) => f,
    };

    let mut list = vec![];
    for line in file.lines() {
        let cmd = Command::from_line(line)?;
        list.push(cmd);
    }

    Ok(list)
}

enum Command {
    Forward(u32),
    Up(u32),
    Down(u32),
}

impl Command {
    fn from_line(line: &str) -> Result<Command, String> {
        let (pref, suf) = match line.split_once(" ") {
            None => return Err(format!("invalid input: {}", line)),
            Some(s) => s,
        };

        let num = match suf.parse::<u32>() {
            Err(e) => return Err(e.to_string()),
            Ok(n) => n,
        };
        let com: Command = match pref {
            "forward" => Forward(num),
            "up" => Up(num),
            "down" => Down(num),
            _ => return Err(format!("invalid input: {}", line)),
        };

        Ok(com)
    }
}
