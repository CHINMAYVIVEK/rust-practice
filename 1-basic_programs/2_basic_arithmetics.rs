// Basic arithmetic operations

fn main() {
    let num1 = 20;
    let num2 = 10;
    println!("Addition: {}", addition(num1, num2));
    println!("Subtraction: {}", subtraction(num1, num2));
    println!("Multiplication: {}", multiplication(num1, num2));
    println!("Division: {}", division(num1, num2));
}

fn addition(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

fn subtraction(num1: i32, num2: i32) -> i32 {
    num1 - num2
}

fn multiplication(num1: i32, num2: i32) -> i32 {
    num1 * num2
}

fn division(num1: i32, num2: i32) -> i32 {
    if num2 > 0 {
        num1 / num2
    } else {
        0
    }
}
