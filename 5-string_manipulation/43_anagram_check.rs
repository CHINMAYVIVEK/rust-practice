fn are_anagrams(s1: &str, s2: &str) -> bool {
    let mut a: Vec<char> = s1.chars().filter(|c| !c.is_whitespace()).collect();
    let mut b: Vec<char> = s2.chars().filter(|c| !c.is_whitespace()).collect();
    a.sort_unstable();
    b.sort_unstable();
    a == b
}

fn main() {
    let s1 = "listen";
    let s2 = "silent";
    println!(
        "Are anagrams ({}, {}): {}",s1,s2,
        are_anagrams(s1, s2)
    );
}
