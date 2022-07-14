use std::io;

fn main() {
    // Ask user what type of tempature conversion they want
    println!("Hello, this program will convert temperatures!");
    println!("Type \"1\" for Celsius to Fahrenheit.");
    println!("Type \"2\" for Celsius to Kelvin.");
    println!("Type \"3\" for Fahrenheit to Celsius.");
    println!("Type \"4\" for Fahrenheit to Kelvin.");
    println!("Type \"5\" for Kelvin to Fahrenheit.");
    println!("Type \"6\" for Kelvin to Celsius.");

    // Create a user input 
    let mut input_unit = String::new();
    io::stdin()
        .read_line(&mut input_unit)
        .expect("Failed to read line");

    // Convert the input to u32
    let int_input_unit: u32 = input_unit.trim().parse().expect("f32 failed");

    // Ask user the value of the unit
    println!("What is the value of the unit?");
    
    // Create a user intput
    let mut input_value = String::new();
    io::stdin()
        .read_line(&mut input_value)
        .expect("Failed to read line");
    
    // Convert the input to f32
    let int_input_value: f32 = input_value.trim().parse().expect("f32 failed");

    // Conditionals, after a valid number is selected
    // the values are computed and printed
    if int_input_unit == 1 {
        println!("{} degrees fahrenheit", celsius_to_fahrenheit(int_input_value));
    } else if int_input_unit == 2 {
        println!("{} degrees kelvin", celsius_to_kelvin(int_input_value));
    } else if int_input_unit == 3 {
        println!("{} degrees celsius", fahrenheit_to_celsius(int_input_value));
    } else if int_input_unit == 4 {
        println!("{} degrees kelivn", fahrenheit_to_kelvin(int_input_value));
    } else if int_input_unit == 5 {
        println!("{} degrees fahrenheit", kelvin_to_fahrenheit(int_input_value));
    } else if int_input_unit == 6 {
        println!("{} degrees celsius", kelvin_to_celsius(int_input_value));
    } else {
        println!("This is not a valid option");
    }
    
}

// Below are the equations used to convert the tempature values
fn celsius_to_fahrenheit(n: f32) -> f32 {
    return 1.8 * n + 32.0; 
}

fn celsius_to_kelvin(n: f32) -> f32 {
    return n + 273.15; 
}

fn fahrenheit_to_celsius(n: f32) -> f32 {
    return (5.0 / 9.0) * (n - 32.0); 
}

fn fahrenheit_to_kelvin(n: f32) -> f32 {
    return (5.0 / 9.0) * (n - 32.0) + 273.15; 
}

fn kelvin_to_fahrenheit(n: f32) -> f32 {
    return 1.8 * (n - 273.15) + 32.0; 
}

fn kelvin_to_celsius(n: f32) -> f32 {
    return n - 273.15; 
}

