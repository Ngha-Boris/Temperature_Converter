pub fn fahrenheit_to_celsius(temperature: f64) -> f64 {
    (temperature - 32.) / 1.8
}

pub fn celsius_to_fahrenheit(temperature: f64) -> f64 {
    temperature * 1.8 + 32.0
}
