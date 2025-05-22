fn is_palindrome(s: &str) -> bool {
    let clean: String = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_lowercase();
    clean == clean.chars().rev().collect::<String>()
}

fn main() {
    let input = "racecar";
    println!("Is '{}' a palindrome? {}", input, is_palindrome("racecar"));
}
