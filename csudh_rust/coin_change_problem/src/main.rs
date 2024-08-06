use std::cmp;

fn change_making(coins: &[usize], n: usize) -> usize {
    let m = coins.len();
    let mut ways = vec![0; n + 1];

    for i in 1..=n {
        let mut temp = usize::MAX;
        let mut j = 0;

        while j < m && i >= coins[j] {
            temp = cmp::min(ways[i - coins[j]], temp);
            j += 1;
        }

        ways[i] = temp + 1;
    }

    ways[n]
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
    println!("Denomiation One, Change-Making: {}", change_making(&denomiation_one, 19));
    let mut timer_after = now.elapsed();
    println!("Time elapsed: {:.2?}", (timer_after - time_before));  

    // Test Case 2:
    time_before = now.elapsed();
    println!("Denomiation Two, Change-Making: {}", change_making(&denomiation_two, 77));
    timer_after = now.elapsed();
    println!("Time elapsed: {:.2?}", (timer_after - time_before)); 
    
    // Test Case 3:
    time_before = now.elapsed();
    println!("Denomiation Three, Change-Making: {}", change_making(&denomiation_three, 25));
    timer_after = now.elapsed();
    println!("Time elapsed: {:.2?}", (timer_after - time_before)); 
    
    // Test Case 4:
    time_before = now.elapsed();
    println!("Denomiation Four, Change-Making: {}", change_making(&denomiation_four, 24));
    timer_after = now.elapsed(); 
    println!("Time elapsed: {:.2?}", (timer_after - time_before)); 
}
