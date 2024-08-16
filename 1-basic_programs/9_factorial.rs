//Calculate the factorial of a number.

fn main() {
    let number = 4;
    println!(
        "factorial of {} is {} by factorial fn ",
        number,
        factorial(number)
    );

    println!(
        "factorial of {} is {} by factorial_recursion fn ",
        number,
        factorial_recursion(number)
    );
}

fn factorial(number: i32) -> i32 {
    let mut res = 1;
    if number <= 1 {
        res = 1;
    } else {
        // for i in 1..=number {
        //     res = i * res;
        // }

        let mut i = 1;
        while i <= number {
            res = i * res;
            i = i + 1;
        }
    }
    return res;
}

fn factorial_recursion(number: i32) -> i32 {
    if number <= 1 {
        1
    } else {
        number * factorial_recursion(number - 1)
    }
}
