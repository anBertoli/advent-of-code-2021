use std::fs;

fn main() {
    println!("Day 3!");
    let values = read_input("input.txt").expect("reading input file");

    let gamma = calc_gamma(&values);
    let epsilon = calc_epsilon(&values);
    println!(
        "part 1, gamma {}, epsilon {}, product {}",
        gamma,
        epsilon,
        gamma * epsilon
    );

    let o2 = calc_o2(&values);
    let co2 = calc_co2(&values);
    println!("part 2, oxygen {}, co2 {}, product {}", o2, co2, o2 * co2);
}

fn read_input(path: &str) -> Result<Vec<u16>, String> {
    let file = match fs::read_to_string(path) {
        Err(err) => return Err(err.to_string()),
        Ok(f) => f,
    };

    let mut list = vec![];
    for line in file.lines() {
        list.push(match u16::from_str_radix(line, 2) {
            Err(err) => return Err(err.to_string()),
            Ok(n) => n,
        });
    }

    Ok(list)
}

fn calc_gamma(values: &[u16]) -> u32 {
    let counts = counts(values);
    let mut gamma = 0;
    for i in 0..12 {
        if counts[i].1 > counts[i].0 {
            gamma = gamma | (1 << i);
        }
    }
    gamma
}

fn calc_epsilon(values: &[u16]) -> u32 {
    let counts = counts(values);
    let mut gamma = 0;
    for i in 0..12 {
        if counts[i].0 > counts[i].1 {
            gamma = gamma | (1 << i);
        }
    }
    gamma
}

fn calc_o2(values: &[u16]) -> u32 {
    let mut values: Vec<u16> = values.to_vec();

    for i in (0..12).rev() {
        let counts = counts(&values);
        if counts[i].0 > counts[i].1 {
            values = values.into_iter().filter(|n| *n & (1 << i) == 0).collect();
        } else {
            values = values.into_iter().filter(|n| *n & (1 << i) > 0).collect();
        };
        if values.len() == 1 {
            return values[0] as u32;
        }
    }

    assert_eq!(values.len(), 1);
    values[0] as u32
}

fn calc_co2(values: &[u16]) -> u32 {
    let mut values: Vec<u16> = values.to_vec();

    for i in (0..12).rev() {
        let counts = counts(&values);

        if counts[i].0 > counts[i].1 {
            values = values.into_iter().filter(|n| *n & (1 << i) > 0).collect();
        } else {
            values = values.into_iter().filter(|n| *n & (1 << i) == 0).collect();
        };
        if values.len() == 1 {
            return values[0] as u32;
        }
    }

    assert_eq!(values.len(), 1);
    values[0] as u32
}

fn counts(values: &[u16]) -> [(u32, u32); 12] {
    let mut counts: [(u32, u32); 12] = [(0, 0); 12];
    for v in values {
        for i in 0..12 {
            if v & ((1 as u16) << i) > 0 {
                counts[i].1 += 1;
            } else {
                counts[i].0 += 1;
            };
        }
    }
    counts
}
