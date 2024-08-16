// area of circle

fn main() {
    let pi = 3.14;

    // if radius is float

    // let r = 2.0;
    // let area = pi * r * r;

    // if radius is int
    let r = 2;
    let area = pi * (r as f64) * (r as f64);

    println!("Area of Circle {}", area);
}
