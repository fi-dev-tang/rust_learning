fn if_condition(){
    let condition = true;
    let number = if condition {
        5
    }else{
        6
    };

    println!("The value of number is {}", number);
}

fn else_if_condition(){
    let n = 6;
    if n % 4 == 0 {
        println!("number is divisible by 4");
    }else if n % 3 == 0 {
        println!("number is divisible by 3");
    }else if n % 2 == 0 {
        println!("number is divisible by 2");
    }else{
        println!("number is not divisible by either 2, 3, 4");
    }
}

/*
总结 for 的三种使用场景
1. 确定在本次循环过后，不再使用该对象，因此可以不使用引用的方式
for item in collection = for item in IntoIterator::into_iter(collection)
转移所有权
[特例] 对于实现了 Copy trait 的数组，可以直接使用本种方式，因为 Copy trait 的数组会进行复制，不影响之后的使用
2. 不可变引用
表示传递给 for 循环的是一个不可变引用，但不能改变数组中的元素等
for item in &collection = for item in collection.iter()
3. 可变引用
不转移所有权，同时对元素内容进行更改
for item in &mut collection = for item in collection.iter_mut()
*/
fn for_condition(){
    for i in 1..=5 {
        println!("{}", i);
    }
}

fn get_index_of_for(){
    let a = [1,2,3,4,5];
    for (i, v) in a.iter().enumerate(){
        println!("{}: {}", i, v);
    }
}

fn continue_and_break(){
    println!("continue case");
    for i in 1..4 {
        if i == 2 {
            continue;
        }
        println!("{}", i);
    }

    println!("break case");
    for i in 1..4 {
        if i == 2 {
            break;
        }
        println!("{}", i);
    }
}


fn while_vs_loop(){
    let mut n = 0;
    while n <= 5 {
        print!("{}! ", n);
        n += 1;
    }
    println!("I JUMP OUT!");

    let mut loop_n = 0;
    loop {
        if loop_n > 5 {
            break;
        }
        print!("{}! ", loop_n);
        loop_n += 1;
    }
    println!("I JUMP OUT AGAIN!");

}

/*
loop 可以作为表达式，返回一个值
break 也可以单独使用，或者作为表达式返回一个值
*/
fn loop_condition(){
    let mut condition = 0;
    let result = loop {
        if condition == 10 {
            break 2 * condition;
        }
        condition += 1;
    };

    println!("result = {}", result);
}

fn main(){
    if_condition();         // if 条件判断
    else_if_condition();    // 多分支条件判断
    for_condition();        // for .. in 集合，集合的三种方式，直接传递，传递引用，传递可变引用

    get_index_of_for();     // 获取 for 的元素下标
    continue_and_break();   // continue 和 break 的不同情况

    while_vs_loop();        // while 和 loop + if + break 组合
    loop_condition();       // loop 的控制条件
}