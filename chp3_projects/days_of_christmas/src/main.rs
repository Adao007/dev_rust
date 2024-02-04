// I believe this is how you DEFINE variables in rust

const FIRST_ITEM: &str = "A partridge in a pear tree"; 
const SECOND_ITEM: &str = "Two turtle doves, and"; 
const THIRD_ITEM: &str = "Three french hens";
const FOURTH_ITEM: &str = "Four calling birds";
const FIFTH_ITEM: &str = "Five golden rings";
const SIXTH_ITEM: &str = "Six geese a-laying";
const SEVENTH_ITEM: &str = "Seven swans a-swimming"; 
const EIGHTH_ITEM: &str = "Eight  maids a-milking";
const NINTH_ITEM: &str = "Nine ladies dancing"; 
const TENTH_ITEM: &str = "Ten lords a-leaping";
const ELEVENTH_ITEM: &str = "Eleven pipers piping";
const TWELFTH_ITEM: &str = "Twelve drummers drumming";

fn main() {

    // Use &str when you have a immutable string 
    // Use String when you have a changing mutable string

    for i in 0..12 {
        println!("[VERSE {}]", i); 
        days(i); 
        
        if i >= 12 {println!("{}", TWELFTH_ITEM); }
        if i >= 11 {println!("{}", ELEVENTH_ITEM);}
        if i >= 10 { println!("{}", TENTH_ITEM); }
        if i >= 9 { println!("{}", NINTH_ITEM); }
        if i >= 8 { println!("{}", EIGHTH_ITEM); }
        if i >= 7 { println!("{}", SEVENTH_ITEM); }
        if i >= 6 { println!("{}", SIXTH_ITEM); }
        if i >= 5 { println!("{}", FIFTH_ITEM); }
        if i >= 4 { println!("{}", FOURTH_ITEM); }
        if i >= 3 { println!("{}", THIRD_ITEM); }
        if i >= 2 { println!("{}", SECOND_ITEM); }
        if i >= 1 { println!("{}", FIRST_ITEM)};

        println!(); 
    }

}

// function for modifying first line of verse
fn days (x: usize) {
    let mut day: &str = "th"; 

    if x == 1 { day = "st"; }
    else if x == 2 { day = "nd"; }
    
    println!("On the {x}{day} day of Christmas, my true love sent to me "); 
}



