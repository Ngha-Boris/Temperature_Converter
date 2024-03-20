use std::io;
mod conversion;

fn main() {
    println!("Hello there, welcome! let's do some conversions 😎😊😊");

    loop {
        let conversion_type = read_input(
            "Please select the conversion type\n
        1.Fahrenheit to Celsius\n
        2.Celsius to Fahrenheit\n
        3.Exit Program",
        )
        .parse::<usize>()
        .expect("Please input either 1, 2, or 3");

        match conversion_type {
            1 => {
                let temperature = read_input("Please input the temperature:");
                let temperature = temperature
                    .parse::<f64>()
                    .expect("Please input a valid temperature");
                let celsius = conversion::fahrenheit_to_celsius(temperature);
                println!("The value in Celsius is: {:.2}°C", celsius);
            }
            2 => {
                let temperature = read_input("Please input the temperature:");
                let temperature = temperature
                    .parse::<f64>()
                    .expect("Please input a valid temperature");
                let fahrenheit = conversion::celsius_to_fahrenheit(temperature);
                println!("The value in Fahrenheit is: {:.2}°F", fahrenheit);
            }
            3 => {
                println!("Thank you for using our program");
                break;
            }
            _ => println!("Please input either 1, 2, or 3"),
        }
    }
}

// Function to read user input
fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}
