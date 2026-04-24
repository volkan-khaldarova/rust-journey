/// Converts Kelvin to Celsius.
///
/// This function subtracts the absolute zero offset (273.15) from the given Kelvin 
/// temperature to find the corresponding Celsius value.
///
/// # Errors
///
/// Returns an `Err` if the provided `kelvin` value is below 0.0 K (Absolute Zero), 
/// as this breaks fundamental physics rules.
///
/// # Examples
///
/// ```
/// let celsius = convert_kelvin_to_celsius(293.15).unwrap();
/// assert_eq!(celsius, 20.0);
/// ```
pub fn convert_kelvin_to_celsius(kelvin: f64) -> Result<f64, &'static str> {
    if kelvin < 0.0 {
        return Err("Kelvin cannot be below zero degrees (Absolute Zero).");
    }
    Ok(kelvin - 273.15)
}

/// Converts Celsius to Fahrenheit.
///
/// Applies the standard conversion formula: `(C * 1.8) + 32.0`.
///
/// # Errors
///
/// Returns an `Err` if the `celsius` value drops below -273.15 °C.
///
/// # Examples
///
/// ```
/// let fahrenheit = convert_celsius_to_fahrenheit(0.0).unwrap();
/// assert_eq!(fahrenheit, 32.0);
/// ```
pub fn convert_celsius_to_fahrenheit(celsius: f64) -> Result<f64, &'static str> {
    if celsius < -273.15 {
        return Err("Celsius cannot be below -273.15 degrees (Absolute Zero).");
    }

    Ok((celsius * 1.8) + 32.0)
}

/// Converts Celsius to the Newton scale.
///
/// Applies the historical Newton conversion formula: `C * (33.0 / 100.0)`.
///
/// # Errors
///
/// Returns an `Err` if the `celsius` value drops below -273.15 °C.
pub fn convert_celsius_to_newton(celsius: f64) -> Result<f64, &'static str> {
    if celsius < -273.15 {
        return Err("Celsius cannot be below -273.15 degrees (Absolute Zero).");
    }

    Ok(celsius * (33.0 / 100.0))
}

// -----------------------------------------------------------------------------
// UNIT TESTS
// -----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    // Brings all functions from the parent module (converter.rs) into the test scope
    use super::*;

    #[test]
    fn test_kelvin_to_celsius_valid() {
        let result = convert_kelvin_to_celsius(273.15).unwrap();
        assert_eq!(result, 0.0); // 273.15 K should be exactly 0.0 C
    }

    #[test]
    fn test_kelvin_to_celsius_invalid_absolute_zero() {
        let result = convert_kelvin_to_celsius(-10.0);
        assert!(result.is_err()); // Must return an Error
    }

    #[test]
    fn test_celsius_to_fahrenheit_valid() {
        let result = convert_celsius_to_fahrenheit(0.0).unwrap();
        assert_eq!(result, 32.0); // 0.0 C should be 32.0 F
    }

    #[test]
    fn test_celsius_to_fahrenheit_invalid_absolute_zero() {
        let result = convert_celsius_to_fahrenheit(-300.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_celsius_to_newton_valid() {
        let result = convert_celsius_to_newton(100.0).unwrap();
        assert_eq!(result, 33.0); 
    }

    
    #[test]
    fn test_celsius_to_newton_invalid_absolute_zero() {
        let result = convert_celsius_to_newton(-274.0);
        assert!(result.is_err());
    }
}