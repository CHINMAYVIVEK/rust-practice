// Generate the first n numbers in the Fibonacci sequence.

fn main() {
    let n = 10; // Generate the first 10 Fibonacci numbers

    // Print the Fibonacci sequence up to n
    for i in 0..n {
        println!("Fibonacci({}): {}", i + 1, fibonacci(i));
    }
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut prev = 0;
    let mut curr = 1;
    let mut result = 0;

    for _ in 2..=n {
        result = prev + curr;
        prev = curr;
        curr = result;
    }

    result
}
