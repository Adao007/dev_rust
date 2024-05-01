fn main() {
    let v: Vec<i32> = Vec::new();
    let v2 = vec![1,2,3];
    let mut v3 = Vec::new();

    v3.push(1);
    v3.push(2);
    v3.push(3);

    // There are two appoarches to reading elements of a vector. 
    // get method or indexing... 
    
    let third: &i32 = &v3[2];
    println!("The element at index 2 (0-2): {third}");
    let third: Option<&i32> = v3.get(2);
    match third {
        Some(third) => println!("Getting third element: {third}"),
        None => println!("There is no third element."),
    }
    
    // Here is how you would iterate over a vector! 
    for i in &v2 {
        println!("{i}");
    }

    // Here is how you would iterate over a mutable vector with
    // evil intentions 
    for i in &mut v3 {
        *i += 50;
        println!("{i}");
    }

    // Notice that you must use the * operator to deference 
    // the iterator variable

    
    // Lets talk about a very cool combo with vectors and enums
    // You can contain a vector of different types by using an
    // enum. 

    enum SpreadsheetCall {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCall::Int(3),
        SpreadsheetCall::Text(String::from("blue")),
        SpreadsheetCall::Float(10.12),
    ];
}

