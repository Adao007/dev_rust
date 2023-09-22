extern crate rand;

use rand::Rng;
use std::io;

fn main() {
    // Create a random number generator
    let mut rng = rand::thread_rng();

    println!("Welcome to the D&D Dice Roller!");

    loop {
        println!("Choose a dice to roll:");
        println!("1. d4");
        println!("2. d6");
        println!("3. d8");
        println!("4. d10");
        println!("5. d12");
        println!("6. d20");
        println!("7. Quit");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "1" => roll_and_print(&mut rng, 4, "d4"),
            "2" => roll_and_print(&mut rng, 6, "d6"),
            "3" => roll_and_print(&mut rng, 8, "d8"),
            "4" => roll_and_print(&mut rng, 10, "d10"),
            "5" => roll_and_print(&mut rng, 12, "d12"),
            "6" => roll_and_print(&mut rng, 20, "d20"),
            "7" => break,
            _ => println!("Invalid choice! Please enter a number from 1 to 7."),
        }
    }

    println!("Thanks for using the D&D Dice Roller!");
}

// Function to roll a specific type of dice and print the result
fn roll_and_print(rng: &mut rand::rngs::ThreadRng, sides: u32, dice_name: &str) {
    let result = roll_dice(rng, sides);
    println!("You rolled a {} and got: {}", dice_name, result);
}

// Function to roll a specific type of dice
fn roll_dice(rng: &mut rand::rngs::ThreadRng, sides: u32) -> u32 {
    // Generate a random number between 1 and the number of sides
    rng.gen_range(1..=sides)
}