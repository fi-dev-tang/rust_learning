/*
尝试错误处理的格式:
在 Option 之上定义 Result 类型
Result<T, E>, 如果成功，返回 Ok(T), 失败，返回 Err(E)
*/
fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
    let first_number = first_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();
    first_number * second_number
}

fn main(){
    let result_1 = multiply("10", "11");
    println!("result_1 = {}", result_1);

    let tt = multiply("t", "22");
    println!("tt = {}", tt);
}