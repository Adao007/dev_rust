use std::io; 
const FIRST_NUMBER: u32 = 0; 
const SECOND_NUMBER: u32 = 1; 

fn main() {
    let fibonacci;
    let mut  number = 0; 

    println!("Welcome to the Fibonacci Sequence Generator!"); 

    while number <= 0 {
        
        number = user_input(); 
        if number <= 0 { println!("Please enter a number larger than 0!")}; 

    }
    
    if number == 1 { fibonacci = FIRST_NUMBER; }
    else if number == 2 { fibonacci = SECOND_NUMBER; }
    else { 
        fibonacci = fib(number - 2); 
    }
    
    println!("The {number}th Fibonacci number is {fibonacci}");

}

fn user_input() -> u32 { 
    let mut number = String::new(); 
    
    println!("Enter a number(larger than 0) to get the nth Fibonacci number");

    io::stdin()
    .read_line(&mut number)
    .expect("Failed to read line"); 

    let number: u32 = number.trim().parse().expect("Enter an int to generate the nth Fibonacci number.");
    
    return number; 
}

fn fib(x: u32) -> u32 {
    let mut count = x; 
    let mut fib: u32 = 0;
    let mut fib2: u32 = 1; 
    let mut result = 0;

    while count > 0 {
        result = fib + fib2;
        fib = fib2;
        fib2 = result; 
        count -= 1;
    }
    return result;
}