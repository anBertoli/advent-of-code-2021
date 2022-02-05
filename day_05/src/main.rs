use std::cmp::Ordering;
use std::fs;
use std::ops::Range;

fn main() {
    println!("Day 5! No fancy things this time.");
    let lines = read_input("input.txt");

    let mut grid = [[0; 1000]; 1000];
    mark_grid_straight_lines(&mut grid, &lines);
    println!(
        "Part 1: straight positions with >= 2: {}",
        count_pos(&mut grid, 2)
    );

    let mut grid = [[0; 1000]; 1000];
    mark_grid_straight_lines(&mut grid, &lines);
    mark_grid_diagonal_lines(&mut grid, &lines);
    println!(
        "Part 2: all positions with >= 2: {}",
        count_pos(&mut grid, 2)
    );
}

#[derive(Debug)]
struct Line {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

fn mark_grid_straight_lines(grid: &mut [[u32; 1000]; 1000], lines: &[Line]) {
    let lines = lines
        .iter()
        .filter(|line| line.x1 == line.x2 || line.y1 == line.y2);

    for line in lines {
        for row in make_range(line.x1, line.x2) {
            for col in make_range(line.y1, line.y2) {
                grid[row as usize][col as usize] += 1;
            }
        }
    }
}

fn mark_grid_diagonal_lines(grid: &mut [[u32; 1000]; 1000], lines: &[Line]) {
    let lines = lines
        .iter()
        .filter(|line| line.x1 != line.x2 && line.y1 != line.y2);

    for line in lines {
        let x_dir = line.x1.cmp(&line.x2);
        let y_dir = line.y1.cmp(&line.y2);
        let mut pos = (line.x1, line.y1);

        loop {
            grid[(pos.0) as usize][(pos.1) as usize] += 1;
            match x_dir {
                Ordering::Less => pos.0 += 1,
                Ordering::Greater => pos.0 -= 1,
                _ => panic!("impossible x order"),
            }
            match y_dir {
                Ordering::Less => pos.1 += 1,
                Ordering::Greater => pos.1 -= 1,
                _ => panic!("impossible y order"),
            }

            if pos == (line.x2, line.y2) {
                // Mark final position and exit.
                grid[(pos.0) as usize][(pos.1) as usize] += 1;
                break;
            }
        }
    }
}

fn make_range(a: u32, b: u32) -> Range<u32> {
    if a <= b {
        a..b + 1
    } else {
        b..a + 1
    }
}

fn count_pos(grid: &mut [[u32; 1000]; 1000], threshold: u32) -> i32 {
    let mut count = 0;
    for row in grid {
        for col in row {
            if *col >= threshold {
                count += 1;
            }
        }
    }
    count
}

fn read_input(path: &str) -> Vec<Line> {
    let file_contents = fs::read_to_string(path).expect("reading file");
    file_contents
        .lines()
        .map(|l| {
            let (pref, suff) = l.split_once(" -> ").unwrap();
            let (x1, y1) = pref.split_once(',').unwrap();
            let (x2, y2) = suff.split_once(',').unwrap();
            let (x1, y1) = (x1.parse().unwrap(), y1.parse().unwrap());
            let (x2, y2) = (x2.parse().unwrap(), y2.parse().unwrap());
            Line { x1, y1, x2, y2 }
        })
        .collect()
}
