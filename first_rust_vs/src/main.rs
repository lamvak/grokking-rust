fn fib(n: i32) -> i32 {
    match n {
        0|1 => 1,
        y   => fib(y - 1) + fib(y - 2)
    }
}

fn main() {
    for i in [1,5, 10].iter() {
        println!("Fib({}) = {}", i, fib(*i));
    }
}
