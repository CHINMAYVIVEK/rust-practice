fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    let a = 24;
    let b = 36;
    println!("num1: {} num2: {} ", a, b);
    let res = lcm(a, b);
    println!("Least Common Multiple (LCM): {}", res);
}
