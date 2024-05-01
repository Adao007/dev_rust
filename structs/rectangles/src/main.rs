// Better use of structs to complete the goal
#[derive(Debug)] // outer attribute allows for debugging and data information
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // The old foolish way of with numerous messy variables
    let foolish_width1 = 30;
    let foolish_height1 = 50;

    // Tuple type to complete the same goal!
    let tuple_rectangle = (30, 50);
    
    // Scale variable used for debugging
    let scale = 2; 

    // Struct type to better name and use data 
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    // Debug case for struct rectangle
    let test_rectangle = Rectangle {
        width: dbg!(30 * scale),
        ..rectangle
    };

    println!(
        "The area of the foolish rectangle is {} square pixels.",
        foolish_area(foolish_width1, foolish_height1)
    );

    println!(
        "The area of the tuple rectangle is {} square pixels.", 
        tuple_area(tuple_rectangle)
    );

    // You can print the details of the struct by using the following
    println!("rectangle is {:?}", rectangle);
    
    dbg!(&test_rectangle);

    println!(
        "The are of the (struct) rectangle is {} square pixels.",
        area(&rectangle)
    );
}

fn foolish_area(width: u32, height:u32) -> u32 {
    width * height
}

fn tuple_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
