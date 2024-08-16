//  Check if a number is even or odd

fn main() {
    let number = 2;
    even_odd(number);
}

fn even_odd(number: i32) {
    if number == 1 || number == 0 {
        println!("exeptional number");
        return;
    }
    if number % 2 == 0 {
        println!("Even number");
        return;
    } else {
        println!("Odd Number");
        return;
    }
}
