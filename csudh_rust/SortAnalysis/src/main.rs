// Anthony Dao
// CSC501 Project 1
// Due: Jul 19, 2024

use rand::Rng;

fn main() {
    use std::time::Instant; 
    let now = Instant::now(); 

    for i in 0..20 { 
        let mut array = create_array(10000);
        let mut elapsed_before = now.elapsed();
       // println!("Time before sorting: {:.2?}", elapsed_before); 

        let comparisons = sort_analysis(&mut array);
        let elapsed = now.elapsed(); 
       // println!("Time after sorting: {:.2?}", elapsed); 
        println!("Sorting Time: {:.2?}", (elapsed - elapsed_before)); 
        println!("Key Comparisons: {}", comparisons);
    }
}

fn sort_analysis(arr: &mut [i32]) -> u32 {
    let mut count = 0; // Initialized counter variable  
    
    for i in 1..(arr.len()) {
        let v = arr[i];
        let mut j = i as isize - 1;
        
        while j as isize >= 0 {
            count += 1;                  // Count comparison incrementation 
            if arr[j as usize] > v {     // Separated the swap condition
                arr[(j + 1) as usize] = arr[j as usize];
                j = (j - 1) as isize;
            } 
            else {
                break;
            }
        }
        arr[(j + 1) as usize] = v;
    }

    count    // Returns count of key comparisons
}

fn create_array(size: usize) -> Vec<i32> {
    let mut random = rand::thread_rng();
    let mut array = Vec::with_capacity(size); 
    for _i in 0..size {
        array.push(random.gen_range(0..100)); 
    }
    array
}

