/*
Constants
Rust has two different types of constants which can be declared in any scope including global.
Both require explicit type annotation:
1. const: An unchangeable value(the common case)
2. static: A possibly mutable variable with 'static lifetime.
The static lifetime is inferred and does not have to be specified.
Accessing or modifying a mutable static variable is unsafe.
*/
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn main(){
    let n = 16;

    // Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) {"big"} else {"small"});
}