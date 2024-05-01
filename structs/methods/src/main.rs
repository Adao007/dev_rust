#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// This impl block refers to the creation of assoicated functions
// Further, methods! In this block, we define an initialization of 
// a square rectangle, shortcutting the need to take in two of the 
// same variables for both width and height! 
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {              // Self keyword in the return type are aliases 
            width: size,    // for the type that appears after the impl keyword!
            height: size,
        }
    }
}

// As you can see, you can have multiple impl blocks! Later chapters will 
// give reasons for the need to do this! However this is correct syntax! 
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Rust uses implicatic typing of pointer methods! 
// p1.distance(&p2);
// (&p1).distance(&p2); 
// are the same! It is a automatic referencing behavior
// that works because methods have a clear receiver! 

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    
    let rect3 = Rectangle::square(39); 

    println!(
        "The area of the retangle is {} square pixels.",
        rect3.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect1? {}", rect2.can_hold(&rect1));
}
