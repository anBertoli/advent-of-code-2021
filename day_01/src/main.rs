use std::process::exit;
use std::fs;


fn main() {
    println!("Day 1!");

    let file = match fs::read_to_string("input.txt") {
        Ok(f) => f,
        Err(err) => {
            println!("{}", err);
            exit(1);
        }
    };

    // First part.
    let increments = match count_increments(&file) {
        Ok(i) => i,
        Err(err) => {
            println!("{}", err);
            exit(1);
        }
    };

    println!("increments: {}", increments);

    // Second part.
    let w_increments = match count_window_increments(&file) {
        Ok(i) => i,
        Err(err) => {
            println!("{}", err);
            exit(1);
        }
    };

    println!("window increments: {}", w_increments);
}

fn count_increments(file: &str) -> Result<i32, String> {
    let mut lines = file.lines();
    let mut prev: i32 = match lines.next() {
        Some(line) => parse_line(line)?,
        _ => return Err("no lines".to_owned()),
    };

    let mut i: i32 = 0;
    for line in file.lines() {
        let n = parse_line(line)?;
        if n > prev {
            i += 1;
        }
        prev = n;
    }

    Ok(i)
}

fn count_window_increments(file: &str) -> Result<i32, String> {
    let mut lines = file.lines();
    let mut window: [i32; 3] = [0; 3];
    let mut sum: i32;

    for i in 0..3 {
        let line: &str = match lines.next() {
            Some(line) => line,
            _ => return Err("no lines".to_owned()),
        };
        let n = parse_line(line)?;
        window[i] = n;
    }
    sum = sum_window(&window);

    let mut i: i32 = 0;
    for line in file.lines() {
        let n = parse_line(line)?;
        window = [window[1], window[2], n];
        let new_sum = sum_window(&window);
        if new_sum > sum {
            i += 1;
        }
        sum = new_sum;
    }

    Ok(i)
}

fn parse_line(line: &str) -> Result<i32, String> {
    match line.parse::<i32>() {
        Ok(n) => Ok(n),
        Err(err) => Err(err.to_string()),
    }
}

fn sum_window(w: &[i32; 3]) -> i32 {
    let mut s = 0;
    for x in w {
        s += x;
    }
    s
}
