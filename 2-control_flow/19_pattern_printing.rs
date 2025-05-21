fn solid_pyramid(n: u32) {
    println!("\nSolid Pyramid:");
    for i in 1..=n {
        let spaces = " ".repeat((n - i) as usize);
        let stars = "*".repeat((2 * i - 1) as usize);
        println!("{}{}", spaces, stars);
    }
}

fn inverted_pyramid(n: u32) {
    println!("\nInverted Pyramid:");
    for i in (1..=n).rev() {
        let spaces = " ".repeat((n - i) as usize);
        let stars = "*".repeat((2 * i - 1) as usize);
        println!("{}{}", spaces, stars);
    }
}

fn half_pyramid(n: u32) {
    println!("\nHalf Pyramid:");
    for i in 1..=n {
        println!("{}", "*".repeat(i as usize));
    }
}

fn inverted_half_pyramid(n: u32) {
    println!("\nInverted Half Pyramid:");
    for i in (1..=n).rev() {
        println!("{}", "*".repeat(i as usize));
    }
}

fn diamond(n: u32) {
    println!("\nDiamond:");
    solid_pyramid(n);
    inverted_pyramid(n - 1);
}

fn number_pyramid(n: u32) {
    println!("\nNumber Pyramid:");
    for i in 1..=n {
        let spaces = " ".repeat((n - i) as usize);
        let nums: String = (1..=i).map(|x| format!("{} ", x)).collect();
        println!("{}{}", spaces, nums.trim_end());
    }
}

fn floyds_triangle(n: u32) {
    println!("\nFloyd's Triangle:");
    let mut num = 1;
    for i in 1..=n {
        for _ in 0..i {
            print!("{} ", num);
            num += 1;
        }
        println!();
    }
}

fn pascals_triangle(n: u32) {
    println!("\nPascal's Triangle:");
    for i in 0..n {
        let mut c = 1;
        print!("{:width$}", "", width = (n - i) as usize);
        for j in 0..=i {
            print!("{:4}", c);
            c = c * (i - j) / (j + 1);
        }
        println!();
    }
}

fn main() {
    let n = 5;

    println!("===============");
    half_pyramid(n);

    println!("===============");
    inverted_half_pyramid(n);

    println!("===============");
    solid_pyramid(n);

    println!("===============");
    inverted_pyramid(n);

    println!("===============");
    diamond(n);

    println!("===============");
    number_pyramid(n);

    println!("===============");
    floyds_triangle(n);

    println!("===============");
    pascals_triangle(n);
    println!("===============");
}
