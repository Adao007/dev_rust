fn fib(n: i32) -> i32 {
    let mut a = 0;
    let mut b = 1;
    let mut c; 

    if n == 0 {
        return a; 
    }
    for _ in 2..=n {
        c = a + b;
        a = b;
        b = c; 
    }

    b
}

fn main() {
    let n = 10;
    println!("{}", fib(n));
}
