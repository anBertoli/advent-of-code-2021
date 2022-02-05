use day_04::Grid;
use std::collections::HashMap;
use std::fs;

fn main() {
    println!("Day 4! This time I went to far...");

    let (inputs, mut grids) = read_input("input.txt").expect("reading input");

    let fastest_grid = find_winning_grid(&mut grids, &inputs, true).expect("fastest grid");
    println!("fastest winning grid result is: {}", fastest_grid);
    reset_grids(&mut grids);
    let slowest_grid = find_winning_grid(&mut grids, &inputs, false).expect("slowest grid");
    println!("slowest winning grid result is: {}", slowest_grid);
}

fn find_winning_grid(grids: &mut [Grid<i32>], inputs: &[i32], fastest: bool) -> Option<i32> {
    let mut final_winner: Option<(usize, i32, i32)> = None;

    for (grid_index, grid) in grids.into_iter().enumerate() {
        let (steps, value) = match mark_grid_until_winning(grid, inputs) {
            Some(v) => v,
            None => continue,
        };

        match final_winner {
            Some(winner) => {
                if fastest && steps < winner.1 {
                    final_winner = Some((grid_index, steps, value));
                }
                if !fastest && steps > winner.1 {
                    final_winner = Some((grid_index, steps, value));
                }
            }
            None => final_winner = Some((grid_index, steps, value)),
        }
    }

    match final_winner {
        Some(winner) => Some(&grids[winner.0].sum_of_unmarked() * winner.2),
        None => None,
    }
}

fn mark_grid_until_winning(grid: &mut Grid<i32>, inputs: &[i32]) -> Option<(i32, i32)> {
    for (steps, input) in inputs.into_iter().enumerate() {
        let winning = grid.set_marked(*input);
        if winning {
            return Some((steps as i32, *input));
        }
    }
    None
}

fn reset_grids(grids: &mut Vec<Grid<i32>>) {
    for grid in grids.into_iter() {
        grid.reset_marks();
    }
}

fn read_input(path: &str) -> Result<(Vec<i32>, Vec<Grid<i32>>), String> {
    let file = match fs::read_to_string(path) {
        Err(err) => return Err(err.to_string()),
        Ok(f) => f,
    };

    let mut lines = file.lines().map(str::trim).filter(|l| !l.is_empty());
    let mut drawn: Vec<i32> = Vec::with_capacity(100);
    let mut boards: Vec<Grid<i32>> = Vec::with_capacity(500);

    // Parse input line.
    let inputs_line = match lines.next() {
        None => return Err("not enough lines".to_string()),
        Some(l) => l,
    };
    for i in inputs_line.split(',') {
        match i.parse::<i32>() {
            Err(err) => return Err(err.to_string()),
            Ok(n) => drawn.push(n),
        };
    }

    // Parse grid lines.
    let mut values = [0; 25];
    let mut i = 0;

    for line in lines {
        for num in line.split(' ').filter(|s| !s.is_empty()) {
            match num.parse::<i32>() {
                Err(err) => return Err(err.to_string()),
                Ok(n) => values[i] = n,
            };
            if i == 24 {
                boards.push(Grid::from_values(&values));
                values = [0; 25];
                i = 0;
            } else {
                i += 1;
            }
        }
    }

    Ok((drawn, boards))
}
