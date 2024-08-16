// swap variables

fn main() {
    let num1 = 20;
    let num2 = 10;

    swap_add_sub(num1, num2);
    println!("==============================");
    swap_mul_div(num1, num2);
    println!("==============================");
    swap_xor(num1, num2);
}

fn swap_add_sub(mut num1: i32, mut num2: i32) {
    println!("initial value of num1: {} and num2: {} ", num1, num2);

    num1 = num1 + num2;
    num2 = num1 - num2;
    num1 = num1 - num2;

    println!("final value of num1: {} and num2: {} ", num1, num2);
}

fn swap_mul_div(mut num1: i32, mut num2: i32) {
    println!("initial value of num1: {} and num2: {} ", num1, num2);

    if num2 == 0 {
        println!("can not divide by 0");
        return;
    } else {
        num1 = num1 * num2;
        num2 = num1 / num2;
        num1 = num1 / num2;

        println!("final value of num1: {} and num2: {} ", num1, num2);
    }
}

fn swap_xor(mut num1: i32, mut num2: i32) {
    println!("initial value of num1: {} and num2: {} ", num1, num2);

    num1 ^= num2;
    num2 ^= num1;
    num1 ^= num2;

    println!("final value of num1: {} and num2: {} ", num1, num2);
}
