fn compress_string(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();

    while let Some(current) = chars.next() {
        let mut count = 1;
        while let Some(&next) = chars.peek() {
            if next == current {
                count += 1;
                chars.next();
            } else {
                break;
            }
        }
        result.push(current);
        result.push_str(&count.to_string());
    }

    result
}

fn main() {
    let input = "aaabbcccccaaa";
    println!("Original String: {}", input);
    println!("Compressed: {}", compress_string(input));
}
