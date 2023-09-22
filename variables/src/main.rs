const THREE_HOURS_IN_SECS: u32 = 60 * 60 * 3; 
fn main() {
    
    // let x = 5; This was the previous code with line 4 that creates an immutable variable x
    
    let x = 5; // The mut makes the variable mutable and tells readers this variable will likely be changed in the future.
    
    let x = x + 1;
    {
        let x = THREE_HOURS_IN_SECS;
        println!(" The value of the inner scope is {x}");
    }
   
    // x = 6; This will give you an error as variables are immutable by default
   
     // This variable it now mutable thus able to be reassigned to a new value 
   
    println!("The value of x is: {x}");

    // Now lets discuss the importance of the let and shadowing. 
    let space = "    "; 
    let space = space.len(); 
    // First we let space equal four white spaces 
    // then we assigned space again but this time by the length of the first variable. 
    // The Shadowing allows us to reuse the same name while changing the datatype of the variables. 

    println!("There should be [4] spaces: number of spaces: {space}");
}
