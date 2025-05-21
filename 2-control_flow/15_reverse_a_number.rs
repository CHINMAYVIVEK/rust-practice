fn reverse_number(mut num: i32) -> i32 {
    let mut reversed = 0;
    while num != 0 {
        reversed = reversed * 10 + num % 10;
        num /= 10;
    }
    reversed
}

fn main() {
    let num = 1234;
    println!("Original number: {}", num);
    let res = reverse_number(num);
    println!("Reversed number: {}", res);
}
