fn main() {
    // Using a function to return the length of the string:s1
    let s1 = String::from("greetings");
    
    let len = calculate_len(&s1);

    println!("The length of '{} is {}.", s1, len);
    // So far the prev lines contain a variable that cannot be modified. 
    // Lets take a look at a variable that can be modified
    let mut s2: String = String::from("Change me");

    change(&mut s2);

    println!("Here is the modified String: {}", s2);

   
}

// the variable s is a reference to s1. 
fn calculate_len(s: &String) -> usize {
    s.len() 
} // Here s goes out of scope. But because it does not have ownership of what it refers to it is not lost.

// function that changes the passed variable
fn change(s: &mut String) {
    s.push_str(", okay I will mold you!"); 
}

/*
    Lets look at a dangling reference problem
    fn dangle() -> &String {
        let s = String::from("problem"); 
        &s // Here we are returning a reference to a variable that will soon go out of scope

    }
*/

// instead lets look at function that will work
fn working_dangle() -> String {
    let s = String::from("working");

    s
}
// Ownership is moved out! 