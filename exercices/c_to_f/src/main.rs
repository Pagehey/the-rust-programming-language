use std::io;

fn main() {
    println!("What is the temperature on Celsius scale?");

    let mut celsius = String::new();

    io::stdin()
        .read_line(&mut celsius)
        .expect("Failed to read line!");

    let celsius: f32 = celsius.
        trim().
        parse().
        expect("Not a number!");

    let fahrenheit = celsius_to_fahrenheit(celsius);

    println!("{} on Celsius scale equals {} Fahrenheit.", celsius, fahrenheit);
}

fn celsius_to_fahrenheit(celsius: f32) -> f32 {
    (celsius * 9.0 / 5.0) + 32.0
}
