// calculate simple interest
// si = (PXRXT)/100

fn main() {
    let principal = 1000.0; // Principal amount
    let rate = 5.0; // Annual interest rate in percentage
    let time = 3.0; // Time in years

    let interest = simple_interest(principal, rate, time);

    println!("Simple Interest: {}", interest);
}

fn simple_interest(principal: f64, rate: f64, time: f64) -> f64 {
    (principal * rate * time) / 100.0
}
