// Determine if a given year is a leap year.

fn main() {
    let year = 2024;

    if is_leap_year(year) {
        println!("Leap Year");
    } else {
        println!("Not a Leap Year");
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}
