//Check if a number is prime.

fn main() {
    let number = 171;

    if is_prime(number) {
        println!("Number {} is prime ", number);
    } else {
        println!("Number {} is not prime ", number);
    }
}

fn is_prime(number: i32) -> bool {
    if number <= 1 {
        return false;
    }

    let mut i = 2;
    while i * i <= number {
        if number % i == 0 {
            return false;
        }
        i += 1;
    }

    true
}
