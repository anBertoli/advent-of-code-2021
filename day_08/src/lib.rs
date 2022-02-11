pub mod digits;

// ⚠️ Some experiments here. ⚠️

pub fn calculate_occurrences(inputs: &[([&str; 10], [&str; 4])]) -> u32 {
    <&[([&str; 10], [&str; 4])] as IntoIterator>::into_iter(inputs)
        .flat_map(|(_, number)| number.into_iter())
        .map(|x| digits::from_segments(*x))
        .filter(|d| d.is_some())
        .count() as u32
}

pub fn add_decoded_numbers(inputs: &[([&str; 10], [&str; 4])]) -> u64 {
    inputs
        .iter()
        .map(|x| solve_row(&x.0[..], &x.1[..]))
        .fold(0, <u64 as std::ops::Add<u64>>::add)
}

fn solve_row(signals: &[&str], numbers: &[&str]) -> u64 {
    let mut remaining: Vec<&str> = signals.into_iter().map(|s| *s).collect();

    // These ones are easy to find, they have a unique length
    // in terms of segments. Great start.
    let (i_one, s_one) = find(&remaining, 1);
    remaining.swap_remove(i_one);
    let (i_four, s_four) = find(&remaining, 4);
    remaining.swap_remove(i_four);
    let (i_seven, s_seven) = find(&remaining, 7);
    remaining.swap_remove(i_seven);
    let (i_eight, s_eight) = find(&remaining, 8);
    remaining.swap_remove(i_eight);

    // The digit 6 have len = 6 and its segments set is the only one
    // of len 6 that doesn't contains the set of the digit 1.
    let (i_six, s_six) = find_six(&remaining, s_one);
    remaining.swap_remove(i_six);

    // The digit 9 has len 6 and we already excluded 6, so only zero have
    // also len 6. But the difference is that the set of segments of the
    // digit 9 contains entirely the set of segments of four.
    let (i_nine, s_nine) = find_nine(&remaining, s_four);
    remaining.swap_remove(i_nine);

    // Then 0 is the last digit with 6 segments. Easy to find.
    let (i_zero, s_zero) = find_zero(&remaining);
    remaining.swap_remove(i_zero);

    // The set of segments of the number three is the only one remained
    // that contains the segments of the digit one.
    let (i_three, s_three) = find_three(&remaining, s_one);
    remaining.swap_remove(i_three);

    // The number 5 have segments completely contained in the ones of the
    // number 6 (not true for 2).
    let (i_five, s_five) = find_five(&remaining, s_six);
    remaining.swap_remove(i_five);

    // Remaining is digit two.
    let (i_two, s_two) = (0, remaining[0]);
    remaining.swap_remove(i_two);

    // Parse final number.
    numbers
        .into_iter()
        .fold(String::new(), |mut acc, s| {
            match *s {
                v if compare_chars(s_zero, v) => acc.push('0'),
                v if compare_chars(s_one, v) => acc.push('1'),
                v if compare_chars(s_two, v) => acc.push('2'),
                v if compare_chars(s_three, v) => acc.push('3'),
                v if compare_chars(s_four, v) => acc.push('4'),
                v if compare_chars(s_five, v) => acc.push('5'),
                v if compare_chars(s_six, v) => acc.push('6'),
                v if compare_chars(s_seven, v) => acc.push('7'),
                v if compare_chars(s_eight, v) => acc.push('8'),
                v if compare_chars(s_nine, v) => acc.push('9'),
                v => panic!("{} should not happen", v),
            }
            acc
        })
        .parse()
        .unwrap()
}

fn find_zero<'a>(signals: &[&'a str]) -> (usize, &'a str) {
    signals
        .iter()
        .map(|s| *s)
        .enumerate()
        .find(|(_, s)| s.len() == 6)
        .expect("to find zero")
}

fn find_six<'a>(signals: &[&'a str], chars_one: &str) -> (usize, &'a str) {
    let mut chars = chars_one.chars();
    let (first, second) = (chars.next().unwrap(), chars.next().unwrap());
    signals
        .iter()
        .map(|s| *s)
        .enumerate()
        .find(|(_, s)| s.len() == 6 && (s.contains(first) ^ s.contains(second)))
        .expect("to find six")
}

fn find_five<'a>(signals: &[&'a str], chars_six: &str) -> (usize, &'a str) {
    signals
        .iter()
        .map(|s| *s)
        .enumerate()
        .find(|(_, s)| {
            if s.len() != 5 {
                return false;
            }
            for s in s.chars() {
                if !chars_six.contains(s) {
                    return false;
                }
            }
            true
        })
        .expect("to find five")
}

fn find_nine<'a>(signals: &[&'a str], chars_four: &str) -> (usize, &'a str) {
    signals
        .iter()
        .map(|s| *s)
        .enumerate()
        .find(|(_, s)| {
            if s.len() != 6 {
                return false;
            }
            for ch in chars_four.chars() {
                if !s.contains(ch) {
                    return false;
                }
            }
            true
        })
        .expect("to find nine")
}

fn find_three<'a>(signals: &[&'a str], chars_one: &str) -> (usize, &'a str) {
    signals
        .iter()
        .map(|s| *s)
        .enumerate()
        .find(|(_, s)| {
            if s.len() != 5 {
                return false;
            }
            for ch in chars_one.chars() {
                if !s.contains(ch) {
                    return false;
                }
            }
            true
        })
        .expect("to find three")
}

fn find<'a>(signals: &[&'a str], digit: u32) -> (usize, &'a str) {
    signals
        .into_iter()
        .map(|s| *s)
        .enumerate()
        .find(|(_, s)| digits::from_segments(s) == Some(digit))
        .expect("not found")
}

#[inline]
fn compare_chars(s1: &str, s2: &str) -> bool {
    s1.len() == s2.len() && s2.chars().all(|c| s1.contains(c))
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_find() {
        let mut signals = [
            "acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb", "ab",
        ];
        let mut numbers = ["cdfeb", "fcadb", "cdfeb", "cdbaf"];
        let num = solve_row(&signals, &numbers);
        assert_eq!(num, 5353);
    }
}
