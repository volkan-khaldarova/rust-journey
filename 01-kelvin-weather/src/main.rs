
mod converter;

use std::io;

fn main() {
    println!("=== Enterprise Temperature Converter ===");
    println!("Please enter the temperature in Kelvin:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line from standard input.");

    let initial_kelvin: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("System Error: Please enter a valid numeric value.");
            return;
        }
    };

    println!("\n--- Results ---");

    let celsius = match converter::convert_kelvin_to_celsius(initial_kelvin) {
        Ok(val) => {
            println!("{:.2} K = {:.2} °C", initial_kelvin, val);
            val
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    if let Ok(val) = converter::convert_celsius_to_fahrenheit(celsius) {
        println!("{:.2} °C = {:.2} °F", celsius, val);
    }

    if let Ok(val) = converter::convert_celsius_to_newton(celsius) {
        println!("{:.2} °C = {:.2} °N", celsius, val);
    }
}