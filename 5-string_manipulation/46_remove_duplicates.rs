fn remove_duplicates(s: &str) -> String {
    let mut seen = [false; 256];
    let mut result = String::new();

    for ch in s.chars() {
        let idx = ch as usize;
        if !seen[idx] {
            seen[idx] = true;
            result.push(ch);
        }
    }

    result
}
fn main() {
    let input = "programming";
    println!("Remove duplicates: {}", remove_duplicates(input));
}
