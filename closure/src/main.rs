fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut burrows_mutably = || list.push(7);
    
    burrows_mutably();
    println!("After calling closure: {list:?}");
}
