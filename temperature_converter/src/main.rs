use colored::*;
use std::io;
fn main() {
    println!("🌡️Temperature converter");

    println!("Choose conversion:");
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");
    println!("3. Celsius to Kelvin");
    println!("4. Kelvin to Celsius");
    println!("5. Fahrenheit to Kelvin");
    println!("6. Kelvin to Fahrenheit");

    loop {
        println!("Enter a number from 1 - 6 to select your conversion:");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Please enter a valid number!".red());
                continue;
            }
        };

        let mut temp_value = String::new();
        println!("Enter the temperature value you want to convert from");

        io::stdin()
            .read_line(&mut temp_value)
            .expect("Failed to read input");

        let temp_value: f64 = match temp_value.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Please enter a valid number!".red());
                continue;
            }
        };

        match input {
            1 => {
                let conversion = celsius_to_fahrenheit(temp_value);
                println!("{}", format!("The conversion result is: 🌡️ {:.2}°C = {:.2}°F", temp_value, conversion).green());
            }
            2 => {
                let conversion = fahrenheit_to_celcius(temp_value);
                println!("{}", format!("The conversion result is: 🌡️ {:.2}°F = {:.2}°C", temp_value, conversion).green());
            }
            3 => {
                let conversion = celsius_to_kelvin(temp_value);
                println!("{}", format!("The conversion result is: 🌡️ {:.2}°C = {:.2}K", temp_value, conversion).green());
            }
            4 => {
                let conversion = kelvin_to_celsius(temp_value);
                println!("{}", format!("The conversion result is: 🌡️ {:.2}K = {:.2}°C", temp_value, conversion).green());
            }
            5 => {
                let conversion = fahrenheit_kelvin(temp_value);
                println!("{}", format!("The conversion result is: 🌡️ {:.2}°F = {:.2}K", temp_value, conversion).green());
            }
            6 => {
                let conversion = kelvin_to_fahrenheit(temp_value);
                println!("{}", format!("The conversion result is: 🌡️ {:.2}K = {:.2}°F", temp_value, conversion).green());
            }
            _ => {
                println!("{}", "Invalid selection!".red());
            }
        }

        break;
    }
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    if c < -273.15 {
        println!(
            "{}",
            "Error: Temperature cannot be below absolute zero (-273.15°C)".red()
        );
        0.0
    } else {
        c * 1.8
    }
}

fn fahrenheit_to_celcius(f: f64) -> f64 {
    if f < -459.67 {
        println!(
            "{}",
            "Error: Temperature cannot be below absolute zero (-459.67°F)".red()
        );
        0.0
    } else {
        (f - 32.0) * 1.8
    }
}

fn celsius_to_kelvin(c: f64) -> f64 {
    if c < -273.15 {
        println!(
            "{}",
            "Error: Temperature cannot be below absolute zero (-273.15°C)".red()
        );
        0.0
    } else {
        c + 273.15
    }
}

fn kelvin_to_celsius(k: f64) -> f64 {
    if k < 0.0 {
        println!(
            "{}",
            "Error: Kelvin cannot be negative (absolute zero is 0K).red()"
        );
        0.0
    } else {
        k - 273.15
    }
}

fn fahrenheit_kelvin(f: f64) -> f64 {
    if f < -459.67 {
        println!(
            "{}",
            "Error: Temperature cannot be below absolute zero (-459.67°F)".red()
        );
        0.0
    } else {
        (f - 32.0) * (5.0 / 9.0) + 273.15
    }
}

fn kelvin_to_fahrenheit(k: f64) -> f64 {
    (k - 273.15) * 1.8 + 32.0
}
