fn solve_quadratic(a: f64, b: f64, c: f64) {
    let discriminant = b * b - 4.0 * a * c;
    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("Two real roots: {} and {}", root1, root2);
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        println!("One real root: {}", root);
    } else {
        println!("No real roots.");
    }
}

fn main() {
    // x^2 - 3x + 2
    let a = 1.0;
    let b = -3.0;
    let c = 2.0;

    solve_quadratic(a, b, c);
}
