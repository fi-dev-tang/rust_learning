/*
Variable bindings have a scope, and are constrained to live on a block.
A block is a collection of statements enclosed by braces {}.
*/
fn main(){
    let long_lived_binding = 1;

    {
        let short_lived_binding = 2;
        println!("innter short: {}", short_lived_binding);
    }
    // Error! `short_lived_binding` doesn't exist in this scope
    // println!("outer short: {}", short_lived_binding);

    println!("outer long: {}", long_lived_binding);
}