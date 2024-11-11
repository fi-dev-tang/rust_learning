/* 自己定义关于 Err(e) 部分的处理 */
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


fn multiply_v2(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError>{
    first_number_str.parse::<i32>().and_then( |first_number| {
            second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
        }
    )
}

fn print(result: Result<i32, ParseIntError>){
    match result{
        Ok(n) => println!("Success, the number is {}", n),
        Err(e) => println!("Error message: {:?}", e),
    }
}

fn main(){
    let twenty_v1 = multiply_v1("10", "22");
    let twenty_v2 = multiply_v2("10", "22");

    let tt_v1 = multiply_v1("tt", "10");
    let tt_v2 = multiply_v2("tt", "10");

    print(twenty_v1);
    print(twenty_v2);

    print(tt_v1);
    print(tt_v2);
}