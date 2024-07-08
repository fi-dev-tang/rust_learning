// 使用 while let 匹配的例子
fn while_let_example(){
    let mut stack = Vec::new(); // 模拟 栈

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop(){
        println!("{}", top);
    }
}

// 使用 for 循环来匹配
fn for_example(){
    let vec = vec!["hello", "from", "rust", "RUST"];

    for (index, value) in vec.iter().enumerate() {
        println!("({}: {})", index, value);
    }
}

// 函数参数作为模式匹配
fn find_location(&(x, y) : &(i32, i32)){
    println!("location at ({}, {})", x, y);
}

fn function_parameter_match(){
    let point = (1i32, 2i32);
    find_location(&point);
}


// 具体遇到的各种模式匹配的例子
fn main(){
    while_let_example();            // while let
    for_example();                  // for

    function_parameter_match();     // 函数参数作为模式匹配 
}