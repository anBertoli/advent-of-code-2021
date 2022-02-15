use std::collections::HashSet;
use std::fs;
use std::iter::Iterator;

fn main() {
    println!("Day 9! Trying some iterator methods.");

    let height_map = parse_input("input.txt");
    let lowers_sum = count_min_points(&height_map);
    println!("Part 1: {}", lowers_sum);

    let sum_of_greater_basins = count_basins(&height_map);
    println!("Part 2: {}", sum_of_greater_basins);
}

fn count_min_points(height_map: &Vec<Vec<u32>>) -> u32 {
    let mut sum = 0_u32;
    for (r, row) in height_map.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            if is_low_point(height_map, r, c) {
                sum += cell + 1;
            }
        }
    }

    sum
}

fn count_basins(height_map: &Vec<Vec<u32>>) -> u32 {
    let mut largest_basins = vec![0; 3];

    for (r, row) in height_map.iter().enumerate() {
        for (c, _) in row.iter().enumerate() {
            if !is_low_point(height_map, r, c) {
                continue;
            }

            // Ok, we found a low point. Now define its
            // boundaries and check if its greater then
            // previous ones.
            let basin_count = explore_basin(height_map, r, c);
            largest_basins.push(basin_count);
            largest_basins.sort_by(|a, b| b.cmp(a));
            largest_basins.pop();
        }
    }

    largest_basins[0] * largest_basins[1] * largest_basins[2]
}

fn explore_basin(height_map: &Vec<Vec<u32>>, row: usize, col: usize) -> u32 {
    // Helper to find higher neighbors. It returns
    // an iterator not an actual collection.
    let find_higher_neighbors = |&(curr_row, curr_col): &(usize, usize)| {
        gen_neighbors(height_map, curr_row, curr_col)
            .into_iter()
            .flatten() // removes Nones
            .filter(move |(n_row, n_col)| {
                let point = height_map[*n_row][*n_col];
                point != 9 && point >= height_map[curr_row][curr_col]
            })
    };

    // We start with one unvisited point,
    // it is the lower point of the basin.
    let mut basin: HashSet<(usize, usize)> = HashSet::with_capacity(30);
    let mut current_cells: HashSet<(usize, usize)> = HashSet::new();
    current_cells.insert((row, col));

    loop {
        let higher_neighbors = current_cells
            .iter()
            .filter(|step| basin.get(step).is_none())
            .flat_map(find_higher_neighbors)
            .collect();

        basin.extend(&current_cells);
        current_cells = higher_neighbors;
        if current_cells.len() == 0 {
            break;
        }
    }

    basin.len() as u32
}

fn is_low_point(height_map: &Vec<Vec<u32>>, row: usize, col: usize) -> bool {
    let neighbors = gen_neighbors(height_map, row, col);
    neighbors
        .into_iter()
        .flatten()
        .all(|(r, c)| height_map[r as usize][c as usize] > height_map[row][col])
}

fn gen_neighbors(
    height_map: &Vec<Vec<u32>>,
    row: usize,
    col: usize,
) -> [Option<(usize, usize)>; 4] {
    let row_max = height_map.len() - 1;
    let col_max = height_map[0].len() - 1;
    let mut ret = [None, None, None, None];
    if row > 0 {
        ret[0] = Some((row - 1, col));
    };
    if col > 0 {
        ret[1] = Some((row, col - 1));
    }
    if col < col_max {
        ret[2] = Some((row, col + 1));
    }
    if row < row_max {
        ret[3] = Some((row + 1, col));
    }
    ret
}

fn parse_input(path: &str) -> Vec<Vec<u32>> {
    let file_contents = fs::read_to_string(path).unwrap();
    let mut grid = vec![];
    for line in file_contents.lines() {
        let parsed_line = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        grid.push(parsed_line);
    }
    grid
}
