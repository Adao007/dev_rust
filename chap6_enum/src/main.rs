fn main() {
    
// We can define enums like this instead of having a complex struct to organize this data!
    enum Quotes {
        Part1(String),
        Part2(String),
        Part3(String),
    }
    
    let first = Quotes::Part1(String::from("Don't let "));
    let second = Quotes::Part2(String::from("me "));
    let third = Quotes::Part3(String::from("keep you."));

// Enums variants can hold different types/amounts of associated data! 
    enum Inventory {
        Jupi(String, String, String),
        Tony(String, String),
    }

    let jupi_bag = Inventory::Jupi(String::from("pen"), String::from("paper"), String::from("key"));
    let tony_pocket = Inventory::Tony(String::from("laptop"), String::from("wallet"));
    
    
// Match Section Start
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    } 

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }    
    println!("{}", value_in_cents(Coin::Penny));
}


