fn main() {
    
    // let x = 5; This was the previous code with line 4 that creates an immutable variable x
    
    let mut x = 5; // The mut makes the variable mutable and tells readers this variable will likely be changed in the future.
    
    println!("The value of x is: {x}");
   
    // x = 6; This will give you an error as variables are immutable by default
   
    x = 6; // This variable it now mutable thus able to be reassigned to a new value 
   
    println!("The value of x is: {x}");
}
