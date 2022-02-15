use std::fs;

fn main() {
    println!("Day 10!");
    let lines = read_input("input.txt");

    let score = calculate_illegals_score(&lines);
    println!("Part 1! Illegals score: {}", score);

    let score = find_middle_score(&lines);
    println!("Part 2! Middle score: {}", score);
}

fn find_middle_score(lines: &Vec<String>) -> u64 {
    let mut incomplete_lines: Vec<u64> = lines
        .iter()
        .map(|l| compute_stack(l))
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .map(|mut line| complete_stack(&mut line))
        .collect();

    incomplete_lines.sort();
    *incomplete_lines.get(incomplete_lines.len() / 2).unwrap()
}

fn calculate_illegals_score(lines: &Vec<String>) -> u32 {
    let mut score = 0;
    for line in lines {
        let (i, char) = match compute_stack(line) {
            Ok(_) => continue,
            Err(illegal_char) => illegal_char,
        };

        match char {
            '(' | ')' => score += 3,
            '[' | ']' => score += 57,
            '{' | '}' => score += 1197,
            '<' | '>' => score += 25137,
            _ => panic!("should not happen at {}: {}", i, char),
        }
    }

    score
}

fn compute_stack(line: &str) -> Result<Vec<char>, (usize, char)> {
    let mut stack = Vec::with_capacity(100);

    for (i, ch) in line.chars().enumerate() {
        match ch {
            '(' => stack.push('('),
            '[' => stack.push('['),
            '{' => stack.push('{'),
            '<' => stack.push('<'),

            ')' => {
                match stack.pop() {
                    None => return Err((i, ch)),
                    Some(t) => {
                        if t != '(' {
                            return Err((i, ch));
                        }
                    }
                };
            }
            ']' => {
                match stack.pop() {
                    None => return Err((i, ch)),
                    Some(t) => {
                        if t != '[' {
                            return Err((i, ch));
                        }
                    }
                };
            }
            '}' => {
                match stack.pop() {
                    None => return Err((i, ch)),
                    Some(t) => {
                        if t != '{' {
                            return Err((i, ch));
                        }
                    }
                };
            }
            '>' => {
                match stack.pop() {
                    None => return Err((i, ch)),
                    Some(t) => {
                        if t != '<' {
                            return Err((i, ch));
                        }
                    }
                };
            }
            _ => panic!("invalid char: {}", ch),
        }
    }

    Ok(stack)
}

fn complete_stack(stack: &Vec<char>) -> u64 {
    let mut score = 0;

    for i in (0..stack.len()).rev() {
        let last_elem = match stack.get(i) {
            Some(s) => s,
            None => break,
        };

        score *= 5;
        score += match last_elem {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => panic!("should not happen {}", last_elem),
        }
    }

    score
}

fn read_input(path: &str) -> Vec<String> {
    let file_contents = fs::read_to_string(path).unwrap();
    file_contents.lines().map(str::to_owned).collect()
}
