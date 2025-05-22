fn char_frequency(s: &str) -> [usize; 256] {
    let mut freq = [0; 256];
    for ch in s.chars() {
        freq[ch as usize] += 1;
    }
    freq
}

fn main() {
    let input = "banana";
    let char = 'a';
    println!("string : {}", input);
    let freq = char_frequency(input);
    println!("Frequency of '{}': {}", char, freq[char as usize]);
}
