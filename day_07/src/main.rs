use std::fs;

fn main() {
    println!("Day 7!");
    let inputs = read_input("input.txt").expect("invalid input");
    let (best_pos, best_fuel) = find_min_fuel_consumption(&inputs, simple_fuel_consumption);
    println!("Part 1: best pos is {} with fuel {}", best_pos, best_fuel);
    let (best_pos, best_fuel) = find_min_fuel_consumption(&inputs, additive_fuel_consumption);
    println!("Part 2: best pos is {} with fuel {}", best_pos, best_fuel);
}

fn find_min_fuel_consumption(inputs: &[u32], f_consumption: fn(u32, u32) -> u32) -> (u32, u32) {
    let max = inputs.iter().max().unwrap();
    let min = *inputs.iter().min().unwrap();

    let mut best_pos: Option<(u32, u32)> = None;
    for pos in min..max + 1 {
        let fuel: u32 = inputs.iter().fold(0, |acc, n| acc + f_consumption(*n, pos));
        // println!("{} {:?}", fuel, best_pos);
        match best_pos {
            Some(best) if fuel < best.1 => best_pos = Some((pos, fuel)),
            None => best_pos = Some((pos, fuel)),
            _ => {}
        }
    }

    best_pos.unwrap()
}

fn simple_fuel_consumption(start: u32, end: u32) -> u32 {
    if start >= end {
        start - end
    } else {
        end - start
    }
}

fn additive_fuel_consumption(start: u32, end: u32) -> u32 {
    let steps = if start >= end {
        start - end
    } else {
        end - start
    };

    (((steps + 1) as f32 / 2.) * steps as f32) as u32
}

#[test]
fn test_additive_fuel_consumption() {
    assert_eq!(0, additive_fuel_consumption(1, 1));
    assert_eq!(1, additive_fuel_consumption(1, 2));
    assert_eq!(3, additive_fuel_consumption(1, 3));
    assert_eq!(6, additive_fuel_consumption(1, 4));
    assert_eq!(45, additive_fuel_consumption(1, 10));
}

fn read_input(path: &str) -> Result<Vec<u32>, String> {
    let raw_input = match fs::read_to_string(path) {
        Err(err) => return Err(err.to_string()),
        Ok(i) => i,
    };

    let mut parsed_input = Vec::with_capacity(100);
    for r in raw_input.split(',') {
        match r.parse::<u32>() {
            Err(err) => return Err(err.to_string()),
            Ok(num) => parsed_input.push(num),
        };
    }

    Ok(parsed_input)
}
