// This was for lesson 3.5 which the doc wanted to be called "branches"
fn main() {
    let number: i8 = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 3 {
        println!("the number was not 3");
    }   else if number != 2 {
        println!("the number was also not 2");
    } else if number != 1 {
        println!("the number was also not 1");
    } // the following else if did not print because the first if runs

    // be careful with multiple else if statements as it clutters code.
    
    let condition = true; 
    let number = if condition { 5 } else { 6 }; 
    println!("The value of number is now {number}");

    // Lets talk about the loops: there are three of them
    // Loops types: loops, while, and for

    // there was a loop here
    // println!("again!"); // this code will run until you tell the code to stop. Use: ctrl c
    // instead lets make a loop that will stop with keywords
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    }; 

    println!("The result if {result}");

    // Lets work with two loops
    {
        let mut count = 0;
        'counting_up: loop {
            println!("count = {count}");
            let mut remaining = 10; 

            loop {
                println!("remaining = {remaining}");
                if remaining == 9 {
                    break; 
                }
                if count == 2 {
                    break 'counting_up; 
                }
                remaining -= 1;

            }
            count += 1;
            
        }
        println!("End count = {count}");

        // Lets look at the while statement
        {
            let mut number = 3; 

            while number != 0 {
                println!("{number}!");

                number -= 1;
            }

            println!("LIFT-OFF!!!!!");
        }
    }

    // finally lets see the For loop
    {
        let a = [10, 20, 30, 40, 50];
        let mut index = 0;

        while index < 5 {
            println!("the value is: {}", a[index]);

            index += 1;
        }

        println!("This code was produced with a for loop!");

        for element in a {
            println!("the value is: {element}");
        }
    }

    {
        for number in (1..4).rev() {
            println!("{number}!");
        }
        println!("LIFTOFF!!!");
    }
    
}

