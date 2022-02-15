use day_08::*;
use std::fs;

fn main() {
    println!(
        "Day 8! This time i tried different things, so the code could look awkward sometimes."
    );

    let mut buffer = String::new();
    let inputs = read_input("input.txt", &mut buffer);

    println!("Part 1: {}", calculate_occurrences(&inputs));
    println!("Part 2: {}", add_decoded_numbers(&inputs));
}

/// This reads as: the function takes a pointer to a String that lives as long
/// as some lifetime 'a. The references inside the returned Vec live as long as
/// the lifetime 'a of the String reference (that is, as long as the pointed
/// String is not dropped). This means that the returned refs point to the
/// String that the String ref is pointing to.
///
/// This is like passing pointers as functions arguments in C.
fn read_input<'a>(path: &str, file_buffer: &'a mut String) -> Vec<([&'a str; 10], [&'a str; 4])> {
    *file_buffer = fs::read_to_string(path).unwrap();

    let mut inputs = Vec::with_capacity(100);
    for line in file_buffer.lines() {
        // ⚠ Maybe this could be more elegant.
        let (pref, suff) = line.split_once(" | ").unwrap();
        let mut signals = [""; 10];
        let mut output = [""; 4];
        for (i, signal) in pref.split(' ').take(10).enumerate() {
            signals[i] = signal;
        }
        for (i, digit) in suff.split(' ').take(4).enumerate() {
            output[i] = digit;
        }
        inputs.push((signals, output));
    }

    inputs
}
