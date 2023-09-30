use std::io; 

fn main() {
    // We need to convert Temps between Fahrenheit and Celsius
    // Make one function that convert Fahrenheit to Celsius 
    // Make one function that convert Celsius to Fahrenheit
    // Take user input: which system of temperature 
    interface(); 
    enter_temperature(); 

    let fah_test:f32 = 82.0; // functions work
    let cel_test:f32 = 27.0; // functions work 

    let mut temp = fahrenheit_convert(fah_test);
    println!("{temp}");
    temp = celsius_convert(cel_test); 
    println!("{temp}"); 
    
    // remember to add user input 

}   

fn interface() {
    println!("Enter temperature system(f/c): "); 
}

fn enter_temperature() {
    println!("Enter the temperature: ");
}

fn fahrenheit_convert(x: f32) -> f32 {
    return (x - 32.0) * 5.0 / 9.0; 
}

fn celsius_convert(x: f32) -> f32 {
    return (x * 9.0 / 5.0) + 32.0; 
}