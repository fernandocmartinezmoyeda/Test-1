// Constants
const FREEZING_POINT_FAHRENHEIT: f64 = 32.0;

// Function to convert Fahrenheit to Celsius
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_POINT_FAHRENHEIT) * 5.0 / 9.0
}

// Function to convert Celsius to Fahrenheit
fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + FREEZING_POINT_FAHRENHEIT
}

fn main() {
    // Declare a mutable variable for the temperature in Fahrenheit
    let mut temp_f: f64 = 32.0;
    
    // Convert the initial temperature to Celsius and print the result
    let temp_c = fahrenheit_to_celsius(temp_f);
    println!("{:.2}°F is {:.2}°C", temp_f, temp_c);

    // Use a loop to convert and print the next 5 integer temperatures
    for _ in 1..=5 {
        temp_f += 1.0;
        let temp_c = fahrenheit_to_celsius(temp_f);
        println!("{:.2}°F is {:.2}°C", temp_f, temp_c);
    }

    let temp_c_example = 0.0;
    let temp_f_example = celsius_to_fahrenheit(temp_c_example);
    println!("{:.2}°C is {:.2}°F", temp_c_example, temp_f_example);
}