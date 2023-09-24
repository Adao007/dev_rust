use std::io;

const SOME_NUMBER_32BIT: i32 = 32; 
const SOME_NUMBER: i64 = 64; 
const SOME_NUMBER_2: i128 = 128;
fn main() {
    // Starting with integer datatype
    let int_8bit: i8 = 127; 
    let int_16bit: i16 = 255; 
    let int_32bit: i32 = SOME_NUMBER_32BIT; 
    let int_64bit: i64 = SOME_NUMBER; 
    let int_128bit: i128 = SOME_NUMBER_2;
    let underscore_numbers: i32 = 1_000; 
    let float_32bit: f32 = 1.000; 
    let float_64bit: f64 = 2.0000; 

    // I don't believe you can use arth operators on different bit variables
    let comb_int = int_8bit as i16 + int_16bit;
    println!("add results = {}", comb_int);
    let comb_int_double = int_32bit as i64 - int_64bit; 
    println!("sub results = {}", comb_int_double); 
    let new_32  = underscore_numbers as i128; 
    let new_32 = new_32 * int_128bit; 
    print!("multiply results = {}\n", new_32);
    print!("divide results = {float_32bit} / {float_64bit}\n"); // this doesn't actually divide the two numbers! 

    // I understand that this is completely messy but its practice.
    let t = true; 
    let f: bool = false; // with explicit type annotation

    // boolean type variables

    let c = 'z'; // single quote symbols for chars
    let z: char = 'z';

    // Lets move to Tuple Type
    let tup: (i8, i16, i32, i64, i128) = (123, 1234, 12345, 123456, 1234567); // this tup holds different variables of various sizes
    let (x, y, z, a, b) = tup; 
    println!("The value of y is {y}");
    //you can also assess the tup by using the index value of the tup
    let one_two_three = tup.0; 
    print!("{one_two_three}\n");

    //  The array is a container of multiple variables that share the same datatype
    let array = [1, 2, 3, 4, 5]; 
    let months = ["Jan", "Feb", "Mar", "Apr", 
                                "May", "June", "July", "Aug", "Sep", "Oct", "Nov", "Dec"];

    // lets write the explicit type annotation
    let array_two: [i8; 5] = [1, 23, 4, 5, 6]; 
    let array_auto = [3; 6]; // this will make an array with the values 3 six times

    println!("Please enter an array index."); 
    let mut index = String::new(); 

    io::stdin() 
        .read_line(&mut index)
        .expect("Failed to read line"); 

    let index: usize = index    
        .trim()
        .parse()
        .expect("Index entered was not a number"); 

    let element = months[index]; 
    print!("The month at the element at the index {index} is: {element}");
    // here we get a sneak peek of the safe checks that Rust is known for. 
    // When you go out of bounds of an array, rust panics and exits out of the program! 

}
