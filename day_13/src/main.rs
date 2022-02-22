use std::cmp::{max, min};
use std::collections::HashSet;
use std::fmt::Debug;
use std::fs;

fn main() {
    println!("Day 13!");

    // Part 1.
    let (mut dots, fold) = read_input("input.txt");
    fold_paper(&mut dots, &fold, 1);
    println!("Part 1: {} points", dots.len());

    // Part 2.
    fold_paper(&mut dots, &fold, fold.len() as u32);
    println!("Part 2: {} points", dots.len());
    print_paper(&mut dots);
}

fn read_input(path: &str) -> (HashSet<(u32, u32)>, Vec<Fold>) {
    let file_contents = fs::read_to_string(path).unwrap();
    let (dots, folds) = file_contents.split_once("\n\n").unwrap();

    let dots: HashSet<(u32, u32)> = dots
        .lines()
        .map(|line| line.split_once(",").unwrap())
        .map(|x| (x.0.parse().unwrap(), x.1.parse().unwrap()))
        .collect();

    let folds: Vec<Fold> = folds
        .lines()
        .map(|line| line.split(" ").nth(2).unwrap())
        .map(|third| Fold::from_str(third).unwrap())
        .collect();

    (dots, folds)
}

fn fold_paper(dots: &mut HashSet<(u32, u32)>, folds: &Vec<Fold>, nf: u32) {
    let n_cycles = min(nf as usize, folds.len());
    for n in 0..n_cycles {
        match folds[n] {
            Fold::X(x) => fold_x(dots, x),
            Fold::Y(y) => fold_y(dots, y),
        }
    }
}

fn fold_x(dots: &mut HashSet<(u32, u32)>, fold_x: u32) {
    let mut dots_to_flip = vec![];
    for &(x, y) in dots.iter() {
        if x > fold_x {
            dots_to_flip.push((x, y))
        }
    }
    for (x, y) in dots_to_flip {
        let new_x = fold_x - (x - fold_x);
        dots.remove(&(x, y));
        dots.insert((new_x, y));
    }
}

fn fold_y(dots: &mut HashSet<(u32, u32)>, fold_y: u32) {
    let mut dots_to_flip = vec![];
    for &(x, y) in dots.iter() {
        if y > fold_y {
            dots_to_flip.push((x, y))
        }
    }
    for (x, y) in dots_to_flip {
        let new_y = fold_y - (y - fold_y);
        dots.remove(&(x, y));
        dots.insert((x, new_y));
    }
}

fn print_paper(dots: &mut HashSet<(u32, u32)>) {
    let (mut max_x, mut max_y) = (0, 0);
    for d in dots.iter() {
        max_x = max(max_x, d.0);
        max_y = max(max_y, d.1);
    }
    for y in 0..max_y + 1 {
        for x in 0..max_x + 1 {
            if dots.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

#[derive(Debug, Clone)]
enum Fold {
    X(u32),
    Y(u32),
}

impl Fold {
    fn from_str(raw_str: &str) -> Result<Fold, String> {
        let (dir, num) = raw_str.split_once("=").unwrap();
        let num = match num.parse::<u32>() {
            Ok(n) => n,
            Err(e) => return Err(e.to_string()),
        };
        match dir {
            "x" | "X" => Ok(Fold::X(num)),
            "y" | "Y" => Ok(Fold::Y(num)),
            _ => Err("invalid fold direction".to_string()),
        }
    }
}
