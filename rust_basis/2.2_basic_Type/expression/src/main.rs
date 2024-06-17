/*
语句和表达式: 函数体由一系列语句组成，最后一个表达式来返回值
*/
fn add_with_extra(x: i32, y: i32) -> i32{
    let x = x + 1; // 语句
    let y = y + 5; // 语句
    x + y // 表达式
}

fn expression_example(){
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

/* 表达式如果不返回任何值，会隐式地返回一个() */
fn ret_unit_type(){
    let x = 1;
    // if 语句块也是一个表达式，因此可以用于赋值，也可以直接返回
    // 类似三元运算符，在 Rust 中我们可以这样写
    let _y = if x % 2 == 1 {
        "odd"
    } else {
        "even"
    };

    // 或者写成一行
    let _z = if x % 2 == 1 {"odd"} else {"even"};
}

fn main(){
    println!("{} + {} = {}", 1, 2, add_with_extra(1,2));
    expression_example();

    assert_eq!(ret_unit_type(), ());
}