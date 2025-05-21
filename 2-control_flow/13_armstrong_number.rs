// Armstrong Number
// An Armstrong number (also known as a Narcissistic number)
// is a number that is equal to the sum of its own digits,
// each raised to the power of the number of digits in the number.
// For example:

// 153 is an Armstrong number because it has 3 digits,
// and when you sum each digit raised to the power of 3, it gives the number itself:
// 1^3 +5^3 +3^3 =1+125+27=153

fn armstrong_number(number: u32) -> bool {
    let str_number = number.to_string();
    let len_str_number = str_number.len() as u32;
    println!("len {} ", len_str_number);
    let sum: u32 = str_number
        .chars()
        .map(|digit| digit.to_digit(10).unwrap().pow(len_str_number))
        .sum();
    sum == number
}

fn main() {
    let number = 153;
    if armstrong_number(number) {
        println!("Number {} is an armstrong number", number);
    } else {
        println!("Number {} is not an armstrong number", number);
    }
}
