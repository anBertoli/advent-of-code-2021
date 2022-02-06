use std::fs;

fn main() {
    println!("Day 6! This went smoothly!");

    let fish = read_input("input.txt").expect("valid input");
    let fish_80 = simulate_growth(&fish, 80);
    let fish_256 = simulate_growth(&fish, 256);

    println!("Part 1: got {} fish.", fish_80);
    println!("Part 2: got {} fish.", fish_256);
}

fn simulate_growth(fish: &[u32], days: u32) -> u64 {
    let mut calendar: [u64; 9] = [0; 9];
    for fish in fish.into_iter() {
        calendar[*fish as usize] += 1;
    }

    let shift = |cal: &mut [u64; 9]| {
        for i in 1..=8 {
            cal[i - 1] = cal[i];
        }
        cal[8] = 0;
    };

    for _ in 0..days {
        let today_newborns = calendar[0];
        shift(&mut calendar);
        calendar[6] += today_newborns;
        calendar[8] += today_newborns;
    }

    calendar.into_iter().reduce(|acc, n| acc + n).unwrap()
}

fn read_input(path: &str) -> Result<Vec<u32>, String> {
    let file_contents = match fs::read_to_string(path) {
        Err(err) => return Err(err.to_string()),
        Ok(f) => f,
    };

    let mut fishes: Vec<u32> = Vec::new();
    for fish in file_contents.split(',') {
        match fish.parse::<u32>() {
            Err(err) => return Err(err.to_string()),
            Ok(f) => fishes.push(f),
        }
    }

    Ok(fishes)
}
