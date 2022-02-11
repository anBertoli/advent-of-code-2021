const LEN_ONE: u8 = 2; // unique
const LEN_FOUR: u8 = 4; // unique
const LEN_SEVEN: u8 = 3; // unique
const LEN_EIGHT: u8 = 7; // unique

/// Parse a Digit from a string slice, based on the length.
/// To be honest, I wanted to try doc tests 😃
///
/// # Examples
/// ```
/// use day_08::digits::{*};
/// assert_eq!(from_segments("as"), Some(1));
/// assert_eq!(from_segments("asfg"), Some(4));
/// assert_eq!(from_segments("asc"), Some(7));
/// assert_eq!(from_segments("a34678s"), Some(8));
/// assert_eq!(from_segments("a3456"), None);
/// ```
pub fn from_segments(segment: &str) -> Option<u32> {
    match segment.len() as u8 {
        LEN_ONE => Some(1),
        LEN_FOUR => Some(4),
        LEN_SEVEN => Some(7),
        LEN_EIGHT => Some(8),
        _ => None,
    }
}
