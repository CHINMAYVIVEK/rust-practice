// Check if a string is a palindrome
//
fn main() {
    let s = "kayak";
// println!("original string {}",s);
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

// fn is_palindrome(s: &str) -> bool {
//     let bytes = s.as_bytes();
//     let len = bytes.len();
//     let mut left = 0;
//     let mut right = len - 1;

//     while left < right {
//         if bytes[left] != bytes[right] {
//             return false;
//         }
//         left += 1;
//         right -= 1;
//     }

//     true
// }

fn is_palindrome(given_string: &str) -> bool {
    let str_len = given_string.len();
    let mut res: bool = false;
    for i in 0..=str_len / 2 {
        if given_string.chars().nth(i) == given_string.chars().nth(str_len - (i + 1)) {
            res = true;
        } else {
            res = false;
        }
    }

    return res;
}
