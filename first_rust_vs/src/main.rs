fn fib(n: i32) -> i32 {
    match n {
        0|1 => 1,
        y   => fib(y - 1) + fib(y - 2)
    }
}

fn foo(x: &mut i32) {
    println!("from foo: {}", *x)
}
fn foo2(x: &i32) {
    println!("from foo2: {}", *x)
}

fn main() {
    for i in [1,5, 10].iter() {
        println!("Fib({}) = {}", i, fib(*i));
    }
    let mut x = 5;
    let xr = &mut x;       // Ok (creates a mutable ref)
    foo(xr);
    foo(xr);
    // The next lines are supposed to cause error according to
    // https://github.com/nrc/r4cppp/blob/3cea93b7e2c18d131c0ad10b3d4d9c3638aece75/borrowed.md?plain=1#L76
    // however - seems to work fine for me :/
    let xr2 = &x;
    foo2(xr2);
    let xr3 = &mut x;
    foo(xr3);
    x = 15;
    println!("and the new val: {}", x)
}
