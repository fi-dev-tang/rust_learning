/*
元组类型的声明
*/
fn create_tuple(){
    let tup: (i32, f64, u8) = (500, 1.82, 1);
    println!("{:?}", tup);
}

// 解构：元组匹配
fn tuple_match(){
    let tup = (500, 2.4, 1);
    let (_x, y , _z) = tup;
    println!("y = {}", y);

    let five_hundred = tup.0;
    let two_point_four = tup.1;
    let one = tup.2;
    println!("five_hundred:{}, two_point_four:{}, one:{}", five_hundred, two_point_four, one);
}

// tuple 的使用场景: 函数返回值
fn tuple_as_return_value(){
    let s = String::from("Rust by practice");
    let (s2, len) = calculate_length(s);

    println!("{}'s len is {}", s2, len);
}

fn calculate_length(s: String) -> (String, usize){
    let length = s.len();
    (s, length)
}

fn main(){
    create_tuple(); // 元组是用括号将多个类型组合在一起
    tuple_match();  // 元组匹配解构

    tuple_as_return_value(); // 元组作为函数返回值
}