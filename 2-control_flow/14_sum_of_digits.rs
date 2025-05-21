//Sum of Digits: Calculate the sum of the digits of a number.

fn calculate_sum_of_digits(number: u32) -> u32 {
    let str_number = number.to_string();
    let sum: u32 = str_number
        .chars()
        .map(|digit| digit.to_digit(10).unwrap())
        .sum();
    return sum;
}

fn main() {
    let number = 1234;
    let res = calculate_sum_of_digits(number);
    println!("sum of digit of number {} is {}", number, res);
}
