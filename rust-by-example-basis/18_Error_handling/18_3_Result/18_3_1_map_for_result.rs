use std::num::ParseIntError;

fn multiply_v1(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError>{
    match first_number_str.parse::<i32>(){
        Ok(first_number) => {
            match second_number_str.parse::<i32>(){
                Ok(second_number) => {
                    Ok(first_number * second_number)
                },
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e),
    }
}

/*
Luckily, Option's map, and_then, and many other combinators are also implemented for Result.
Result contains a complete listing.
*/
fn multiply_v2(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError>{
    first_number_str.parse::<i32>().and_then(|first_number| {
            second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
        }
    )
}


fn print(result: Result<i32, ParseIntError>){
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main(){
    let twenty = multiply_v1("10", "2");
    print(twenty);

    let tt = multiply_v1("t", "2");
    print(tt);

    let twenty_v2 = multiply_v2("10", "2");
    print(twenty_v2);

    let tt_v2 = multiply_v2("t", "2");
    print(tt_v2);
}