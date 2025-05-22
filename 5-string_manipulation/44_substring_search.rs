fn find_substring(haystack: &str, needle: &str) -> Option<usize> {
    haystack.find(needle)
}

fn main() {
    let haystack = "hello world";
    let needle = "world";
    println!(
        "Find needle '{}' in haystack '{}': {:?}",
        needle,
        haystack,
        find_substring(haystack, needle)
    );
}
