use std::io;
use std::io::Write;

use console::Term;

fn main() {
    let mut temperature: String = String::new();
    let temperature_unit: Term = Term::stdout();
    let result: i32;

    print!("Enter the temperature: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    let temperature: i32 = temperature.trim().parse().expect("Please type a number");

    print!("Do you want to convert to (F)ahrenheit or (C)elsius? ");
    io::stdout().flush().unwrap();

    let temperature_unit: char = temperature_unit.read_char()
        .expect("Failed to read line");

    println!("{temperature_unit}");

    match temperature_unit {
        'F' => result = convert_to_fahrenheit(temperature),
        'C' => result = convert_to_celsius(temperature),
        _ => panic!("Unknown temperature unit: {}", temperature_unit)
    };

    println!("Result: {} {}", result, temperature_unit)
}

fn convert_to_fahrenheit(temperature: i32) -> i32 {
    temperature * 9/5 + 32
}

fn convert_to_celsius(temperature: i32) -> i32 {
    (temperature - 32) * 5/9
}
