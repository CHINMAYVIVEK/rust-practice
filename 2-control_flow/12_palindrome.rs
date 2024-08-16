// Check if a string is a palindrome
//
fn main() {
    let s = "racecar";

    if is_palindrome(s) {
        println!("string {} is a palindrome", s);
    } else {
        println!("string {} is not a palindrome", s);
    }
}

// fn is_palindrome(s: &str) -> bool {
//     let reversed: String = s.chars().rev().collect();
//     s == reversed
// }

fn is_palindrome(s: &str) -> bool {
    let bytes = s.as_bytes();
    let len = bytes.len();
    let mut left = 0;
    let mut right = len - 1;

    while left < right {
        if bytes[left] != bytes[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }

    true
}
