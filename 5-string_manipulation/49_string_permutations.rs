fn permute(s: &mut Vec<char>, l: usize, r: usize) {
    if l == r {
        println!("{}", s.iter().collect::<String>());
    } else {
        for i in l..=r {
            s.swap(l, i);
            permute(s, l + 1, r);
            s.swap(l, i); // backtrack
        }
    }
}

fn main() {
    let input = "abc";
    println!("Permutations of {}:",input);
    permute(&mut input.chars().collect::<Vec<_>>(), 0, 2);
}
