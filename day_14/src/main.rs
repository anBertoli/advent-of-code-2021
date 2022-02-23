use std::collections::HashMap;
use std::fs;

fn main() {
    println!("Day 14!");

    let (template, pairs) = read_input("input.txt");
    let counts = process_polymer_template(&template, &pairs, 10);
    println!("Part 1. Counts: {:?}", counts);

    let counts = process_polymer_template(&template, &pairs, 40);
    println!("Part 2. Counts: {:?}", counts);
}

fn process_polymer_template(template: &str, pairs: &Pairs, iter: u64) -> u64 {
    let mut pairs_counts = HashMap::new();
    let mut chars = template.chars();
    let mut first = chars.next().unwrap();
    *pairs_counts.entry(('_', first)).or_insert(0) += 1;
    loop {
        let second = match chars.next() {
            Some(c) => c,
            None => break,
        };
        *pairs_counts.entry((first, second)).or_insert(0) += 1;
        first = second
    }
    *pairs_counts.entry((first, '_')).or_insert(0) += 1;

    for _ in 0..iter {
        let mut new_counts = HashMap::new();
        for ((first, second), val) in pairs_counts {
            if first == '_' || second == '_' {
                *new_counts.entry((first, second)).or_insert(0) += val;
                continue;
            }
            let between = pairs.get(&(first, second)).unwrap();
            *new_counts.entry((first, *between)).or_insert(0) += val;
            *new_counts.entry((*between, second)).or_insert(0) += val;
        }
        pairs_counts = new_counts;
    }

    calc_on_polymer(&pairs_counts)
}

fn calc_on_polymer(counts: &HashMap<(char, char), u64>) -> u64 {
    let mut c: HashMap<char, u64> = HashMap::new();
    for ((first, second), val) in counts {
        *c.entry(*first).or_insert(0) += val;
        *c.entry(*second).or_insert(0) += val;
    }
    c.remove(&'_');
    let max = c.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
    let min = c.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap();
    (max.1 - min.1) / 2
}

type Pairs = HashMap<(char, char), char>;

fn read_input(path: &str) -> (String, Pairs) {
    let file_contents = fs::read_to_string(path).unwrap();
    let (template, pairs) = file_contents.split_once("\n\n").unwrap();

    let pairs = pairs
        .lines()
        .map(|pair| {
            let chars: Vec<char> = pair.chars().collect();
            ((chars[0], chars[1]), chars[6])
        })
        .collect();

    (template.to_string(), pairs)
}
