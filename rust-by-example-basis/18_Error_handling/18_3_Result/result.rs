/*
Result:
Result is a richer version of the Option type that describes possible error instead of possible absence.
Result<T,E>:
1. Ok<T>: An element T was found
2. Err<E>: An error was found with element E
*/
fn multiply(first_number_str: &str, second_number_str: &str) -> i32{
    // let's try using unwrap() to get the number out.
    let first_number = first_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();
    first_number * second_number
}

fn main(){
    let twenty = multiply("10", "2");
    println!("double is {}", twenty);

    let tt = multiply("t", "2");
    println!("double is {}", tt);
}