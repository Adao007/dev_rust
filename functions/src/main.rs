fn main() {
    println!("Hello, world!");

    separate_function ();
    separate_math_function(9);
    two_parameter_function(98, 'h');
    let convert: i8 = expression_convert(expression_function()); // This is a silly way of doing this.
    println!("The expression function returned the number: {convert}");
}

fn separate_function () {
    println!("Goodbye, cruel world!");
}

fn separate_math_function (x: i8) { //when you input parameters you need to define the type
    println!("The value of x is: {x}"); 
}

fn two_parameter_function (x: i8, measurement: char) {
    println!("The measurement of the value inputted: {x}{measurement}"); 
}

fn expression_function () -> i8 {
    5
}

fn expression_convert (mut x: i8) -> i8
{
    x = expression_function(); 
    return x; 
}