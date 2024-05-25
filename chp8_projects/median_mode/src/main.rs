// Anthony Dao 
// 1. A program that creates a list of randomly generated numbers
// 2. Return the median of the list 
// 3. Return the mode of the list

extern crate rand;
use rand::Rng;
use std::collections::HashMap;

fn main() {
    let mut rng = rand::thread_rng();

    // Initialize List
    let mut list: [u32; 10] = [0; 10];
    let mut vector: Vec<u32> = Vec::new();
    let mut frequency: HashMap<u32, i32> = HashMap::new();

    for i in 0..list.len() {
        list[i] = rng.gen_range(0..100);
    }

    copy_list(&mut list, &mut vector);
    println!("given list: {:?}" ,vector);
    
    vector.sort();
    
    println!("after sorted: {:?}", vector);
    median(&mut vector);
    
}

// function to copy array into vector
fn copy_list(x: &mut [u32; 10], v: &mut Vec<u32>) {
    for number in x {
        v.push(*number);
    }
}

//function for printing out the median
fn median(v: &mut Vec<u32>) {
    let middle = (v.len() + 1) / 2;
    println!("Median: {:?}", v[middle]); 
}

//function to insert elements into HashMap
fn insert_elements(v: &mut Vec<u32>, hm: &mut HashMap<u32, i32>) {
    for element in v {
   println!("{}" ,vector);
        
    }
}












