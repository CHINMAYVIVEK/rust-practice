fn binary_to_decimal(binary: &str) -> u32 {
    u32::from_str_radix(binary, 2).unwrap()
}

fn main() {
    let binary = "1101";
    let res = binary_to_decimal(binary);
    println!("Binary {} to Decimal: {}", binary, res);
}
