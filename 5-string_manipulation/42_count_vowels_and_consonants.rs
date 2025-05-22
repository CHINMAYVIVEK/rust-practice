fn count_vowels_and_consonants(s: &str) -> (usize, usize) {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut vowels_count = 0;
    let mut consonants_count = 0;

    for ch in s.to_lowercase().chars() {
        if ch.is_alphabetic() {
            if vowels.contains(&ch) {
                vowels_count += 1;
            } else {
                consonants_count += 1;
            }
        }
    }

    (vowels_count, consonants_count)
}

fn main() {
    let input = "hello world";
    println!("string : {}", input);
    let (v, c) = count_vowels_and_consonants(input);
    println!("Vowels: {}, Consonants: {}", v, c);
}
