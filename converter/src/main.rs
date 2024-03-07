use std::io;

fn main() {
    println!("Hello there, welcome! lets do some conversions 😎😊😊");
    loop {
        println!("Please select the conversion type");
        println!("1. Fahrenheit to Celsius");
        println!("2. Celsius to Fahrenheit");
        println!("3. Exit Program");

        let mut conversion_type = String::new();
        io::stdin()
            .read_line(&mut conversion_type)
            .expect("Failed to read line");

        let conversion_type: i32 = match conversion_type.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input either 1, 2, or 3");
                continue;
            }
        };

        match conversion_type {
            1 => {
                println!("Please input the temperature: ");
                let mut temperature = String::new();
                    io::stdin().read_line(&mut temperature)
                    .expect("Failed to read line");
                let temperature: f64 = match temperature.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please input a valid temperature");
                        continue;
                    }
                };

                let celsius = (temperature - 32.) / 1.8;
                println!("The value in Celsius is: {celsius}°C");
            }
            2 => {
                println!("Please input the temperature: ");
                let mut temperature = String::new();
                    io::stdin().read_line(&mut temperature)
                    .expect("Failed to read line");
                let temperature: f64 = match temperature.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please input a valid temperature");
                        continue;
                    }
                };

                let  fahrenheit = temperature * 1.8 + 32.0;
                println!("The value in Fahrenheit is: {fahrenheit}°F");
            }
            3 => {
                println!("Thank you for using our program");
                break;
            }
            _ => {
                println!("Please input either 1, 2, or 3");
                continue;
            }
        }
    }
}

