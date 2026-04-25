fn main() {
    
    // 1. Create a variable named myAge, and set it equal to your age as a number.
    let my_age: f64 = 27.0;

    // 2. The first two years of a dog's life count as 10.5 dog years each.
    let mut early_years: f64 = 2.0;

    // 3. Use the multiplication assignment operator to calculate dog years for the early stage.
    early_years *= 10.5;

    // 4. Since we accounted for the first two years, subtract 2 from myAge.
    let mut later_years: f64 = my_age - 2.0;

    // 5. Multiply the remaining years by 4 to calculate the later dog years.
    later_years *= 4.0;

    // 6. Add earlyYears and laterYears together to get the final result.
    let my_age_in_dog_years: f64 = early_years + later_years;

    // 7. Store the name in lowercase using the built-in string method.
    let my_name: String = "Volkan".to_lowercase();

    // 8. Display the final result using string interpolation.
    println!("My name is {my_name}. I am {my_age} years old in human years, which is {my_age_in_dog_years} years old in dog years. ");
}
