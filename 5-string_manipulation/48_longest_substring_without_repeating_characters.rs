fn longest_unique_substring(s: &str) -> String {
    let mut seen = [usize::MAX; 256];
    let (mut start, mut max_len, mut max_start) = (0, 0, 0);

    for (i, ch) in s.chars().enumerate() {
        let idx = ch as usize;
        if seen[idx] != usize::MAX && seen[idx] >= start {
            start = seen[idx] + 1;
        }
        seen[idx] = i;
        if i - start + 1 > max_len {
            max_len = i - start + 1;
            max_start = start;
        }
    }

    s.chars().skip(max_start).take(max_len).collect()
}

fn main() {
    let input = "abcabcbb";
    println!("input string: {}", input);
    println!(
        "Longest unique substring: {}",
        longest_unique_substring(input)
    );
}
