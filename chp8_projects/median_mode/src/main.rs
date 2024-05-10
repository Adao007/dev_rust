// Anthony Dao 
// 1. A program that creates a list of randomly generated numbers
// 2. Return the median of the list 
// 3. Return the mode of the list

extern crate rand;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut n: u32 = rng.gen_range(0..100);
    println!("Random number: {}", n);

    // Initialize List
    let mut list: [u32; 10] = [0; 10]; 
    for i in 0..list.len() {
        n = rng.gen_range(0..100);
        list[i] = n;
    }

    println!("{:?}", list);
    
}
