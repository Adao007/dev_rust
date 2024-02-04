fn main() {
    // Variable for if statement 
    let number = -5; 

    // IF Control Structure
    if number < 0 { println!("Number is negative!"); } 
    else { println!("Number is positive!"); }
    // IF Control Structure Part 2: ELSE IF
    if number != 5 {println!("Number is not 5!"); }
    else if number != 4 {println!("Number is also not 4!"); }
    else if number != 3 {println!("Number is also not 3!")} 
    //Only the first if prints, obv how control structures work 

    // You can set variables depending on bool conditions. 
    let condition = true; 
    let number = if condition {35} else {26}; 
    println!("Number now equals {number}"); 

    // Lets talk about looping in rust
    let mut counter = 0; 
    let result = loop {
        counter += 1; 

        if counter == 100 {
            break; 
        }
    }; 

    
    // Now lets use the counter variable to work with a double loop. 
    'outer_loop: loop { // We are using the label of outer_loop to use a second loop 
        println!("count = {counter}"); 
        let mut inner_counter = 0; 

        loop {
            println!("inner_counter = {inner_counter}"); 
            if inner_counter == 10 {
                break; 
            }

            if counter == 0 {
                break 'outer_loop; 
            }
            inner_counter += 1; 
        }
        counter -= 1; 
    }

// While loop
    {
        while counter != 100 {
            println!("counter = {counter}"); 
            counter += 1; 
        }
    }

    {
        // arrays use [] for their elements 
        let array = [0, 1, 2, 3, 4, 5, 6]; 
        let mut item = 0; 

        for element in array {
            println!("{element}")
        }
    }

{    
    for number in (1..5){
        println!("{number}");
    }
}

}

