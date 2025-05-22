fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    let input = "hello world";
    println!("Reverse: {}", reverse_string(input));
}
