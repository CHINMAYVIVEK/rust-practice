// Temperature converter between Celsius, Fahrenheit, and Kelvin

fn main() {
    let celsius = 25.0;
    let fahrenheit = 77.0;
    let kelvin = 298.15;

    println!("{} °C to °F: {} °F", celsius, c_2_f(celsius));
    println!("{} °C to K: {} K", celsius, c_2_k(celsius));
    println!("{} °F to °C: {} °C", fahrenheit, f_2_c(fahrenheit));
    println!("{} °F to K: {} K", fahrenheit, f_2_k(fahrenheit));
    println!("{} K to °C: {} °C", kelvin, k_2_c(kelvin));
    println!("{} K to °F: {} °F", kelvin, k_2_f(kelvin));
}

fn c_2_f(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn c_2_k(celsius: f64) -> f64 {
    celsius + 273.15
}

fn f_2_c(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn f_2_k(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0 + 273.15
}

fn k_2_c(kelvin: f64) -> f64 {
    kelvin - 273.15
}

fn k_2_f(kelvin: f64) -> f64 {
    (kelvin - 273.15) * 9.0 / 5.0 + 32.0
}
