pub fn fahrenheit_to_celsius(temperature: f64) -> f64 {
    (temperature - 32.) / 1.8
}

pub fn celsius_to_fahrenheit(temperature: f64) -> f64 {
    temperature * 1.8 + 32.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fahrenheit_to_celsius() {
        assert_eq!(fahrenheit_to_celsius(32.0), 0.0);
        assert_eq!(fahrenheit_to_celsius(212.0), 100.0);
        assert_eq!(fahrenheit_to_celsius(68.0), 20.0);
    }

    #[test]
    fn test_celsius_to_fahrenheit() {
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
        assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
        assert_eq!(celsius_to_fahrenheit(20.0), 68.0);
    }
}
