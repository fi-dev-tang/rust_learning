/* Variable bindings are immuable by default, but this can be overriden using the mut modifier. */
fn main(){
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation:{}", mutable_binding);

    mutable_binding += 1;
    println!("After mutation:{}", mutable_binding);
}