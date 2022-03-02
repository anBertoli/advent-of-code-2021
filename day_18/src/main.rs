use std::fmt::{Formatter, Write};
use std::fs;

fn main() {
    println!("Day 18!");

    let nums = parse_input("input.txt");
    println!("{:?}", nums);
    let mut sum = add_numbers(nums.clone());
    let magnitude = calc_magnitude(&mut sum);
    println!("Part 1: number: {:?}, magnitude: {}", sum, magnitude);

    let max_magnitude = calc_highest_sum_pair(nums);
    println!("Part 2: max magnitude: {}", max_magnitude);
}

fn calc_highest_sum_pair(nums: Vec<Num>) -> u32 {
    let mut max = 0;
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if i == j {
                continue;
            }

            let mut sum = Num::Pair(Box::new((nums[i].clone(), nums[j].clone())));
            reduce_number(&mut sum);
            let magn = calc_magnitude(&mut sum);
            if magn > max {
                max = magn;
            }
        }
    }
    max
}

fn calc_magnitude(num: &mut Num) -> u32 {
    match num {
        Num::Pair(pair) => 3 * calc_magnitude(&mut pair.0) + 2 * calc_magnitude(&mut pair.1),
        Num::Val(v) => *v,
    }
}

fn add_numbers(nums: Vec<Num>) -> Num {
    nums.into_iter()
        .reduce(|acc, num| {
            let mut sum = Num::Pair(Box::new((acc, num)));
            reduce_number(&mut sum);
            sum
        })
        .unwrap()
}

fn reduce_number(num: &mut Num) {
    loop {
        if let Some(_) = explode_pair(num, 0) {
            continue;
        }
        if let Some(_) = split_number(num) {
            continue;
        }
        break;
    }
}

fn split_number(num: &mut Num) -> Option<()> {
    match num {
        Num::Pair(pair) => {
            if let Some(_) = split_number(&mut pair.0) {
                return Some(());
            };
            if let Some(_) = split_number(&mut pair.1) {
                return Some(());
            }
            None
        }
        Num::Val(v) => {
            if *v >= 10 {
                let left: u32 = *v / 2;
                *num = Num::Pair(Box::new((Num::Val(left), Num::Val(*v - left))));
                Some(())
            } else {
                None
            }
        }
    }
}

fn explode_pair(num: &mut Num, depth: u32) -> Option<(u32, bool, u32, bool)> {
    let pair: &mut (Num, Num) = match num {
        Num::Val(_) => return None,
        Num::Pair(pair) => pair,
    };

    // We find the pair to explode only if the depth is
    // greater than and both arms are literal values.
    if depth >= 4 {
        if let (&Num::Val(l), &Num::Val(r)) = (&pair.0, &pair.1) {
            *num = Num::Val(0);
            return Some((l, false, r, false));
        }
    }

    // We got it from left, so we can add to
    // the leftmost value of the right branch.
    if let Some(mut exploded_pair) = explode_pair(&mut pair.0, depth + 1) {
        if exploded_pair.3 {
            return Some(exploded_pair);
        }

        add_to_leftmost(&mut pair.1, exploded_pair.2);
        exploded_pair.3 = true;
        return Some(exploded_pair);
    }

    // We got it from right, so we can add to
    // the rightmost value of the left branch.
    if let Some(mut exploded_pair) = explode_pair(&mut pair.1, depth + 1) {
        if exploded_pair.1 {
            return Some(exploded_pair);
        }

        add_to_rightmost(&mut pair.0, exploded_pair.0);
        exploded_pair.1 = true;
        return Some(exploded_pair);
    }

    None
}

fn add_to_leftmost(num: &mut Num, val: u32) {
    match num {
        Num::Pair(pair) => add_to_leftmost(&mut pair.0, val),
        Num::Val(v) => *v += val,
    }
}

fn add_to_rightmost(num: &mut Num, val: u32) {
    match num {
        Num::Pair(pair) => add_to_rightmost(&mut pair.1, val),
        Num::Val(v) => *v += val,
    }
}

#[derive(Clone)]
enum Num {
    Val(u32),
    Pair(Box<(Num, Num)>),
}

impl std::fmt::Debug for Num {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Num::Val(v) => f.write_fmt(format_args!("{}", v))?,
            Num::Pair(pair) => {
                f.write_char('[')?;
                (*pair).0.fmt(f)?;
                f.write_char(',')?;
                (*pair).1.fmt(f)?;
                f.write_char(']')?;
            }
        };
        Ok(())
    }
}

fn parse_input(path: &str) -> Vec<Num> {
    let file_contents = fs::read_to_string(path).unwrap();
    file_contents
        .lines()
        .map(|mut line| parse_num(&mut line))
        .collect()
}

fn parse_num(str_num: &mut &str) -> Num {
    match str_num.chars().next().unwrap() {
        '[' => {
            // Discard "[" and parse left, discard "," and
            // parse right, finally discard "]".
            *str_num = &str_num[1..];
            let left = parse_num(str_num);
            *str_num = &str_num[1..];
            let right = parse_num(str_num);
            *str_num = &str_num[1..];
            Num::Pair(Box::new((left, right)))
        }
        _ => {
            // Discard literal number and ",".
            let str_n = str_num.chars().next().unwrap();
            let n = str_n.to_digit(10).unwrap();
            *str_num = &str_num[1..];
            Num::Val(n)
        }
    }
}
