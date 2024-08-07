use std::cmp;                           // Used for Min function

fn change_making(coins: &[usize], n: usize) -> usize {
    let c = coins.len();                // Variable to keep track of length of Coins array
    let mut min = vec![0; n + 1];       // Creates new array to keep track of min coins to return change

    for i in 1..=n {                    // For loop 1 to change amount
        let mut temp = usize::MAX;      // Temp is set to largest usize value
        let mut j = 0;                  // Used for while loop increment

        while j < c && i >= coins[j] {
            temp = cmp::min(min[i - coins[j]], temp);
            j += 1;
        }

        min[i] = temp + 1;
    }

    min[n]
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let denomiation_one = vec![1, 3, 4];            // Change amount: 19
    let denomiation_two = vec![1, 5, 10, 20, 25];   // Change amount: 77
    let denomiation_three = vec![1, 4, 7, 11];      // Change amount: 25
    let denomiation_four = vec![1, 5, 7, 13];       // Change amount: 24 
    
    // Test Case 1: 
    let mut time_before = now.elapsed();
    println!("Denomiation One: (1, 3, 4), Minimum Coins needed to return change of 19: {}"
             , change_making(&denomiation_one, 19));
    let mut timer_after = now.elapsed();
    println!("Time elapsed: {:.2?}", (timer_after - time_before));  

    // Test Case 2:
    time_before = now.elapsed();
    println!("Denomiation Two: (1, 5, 10, 20, 25), Minimum Coins needed to return change of 77: {}"
             , change_making(&denomiation_two, 77));
    timer_after = now.elapsed();
    println!("Time elapsed: {:.2?}", (timer_after - time_before)); 
    
    // Test Case 3:
    time_before = now.elapsed();
    println!("Denomiation Three: (1, 4, 7, 11), Minimum Coins needed to return change of 25: {}"
             , change_making(&denomiation_three, 25));
    timer_after = now.elapsed();
    println!("Time elapsed: {:.2?}", (timer_after - time_before)); 
    
    // Test Case 4:
    time_before = now.elapsed();
    println!("Denomiation Four: (1, 5, 7, 13), Minimum Coins needed to return change of 24: {}"
             , change_making(&denomiation_four, 24));
    timer_after = now.elapsed(); 
    println!("Time elapsed: {:.2?}", (timer_after - time_before)); 
}
