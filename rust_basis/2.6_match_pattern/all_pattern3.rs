/* 全模式列表: 从使用嵌套的 _ 忽略部分值开始 */
fn customized_change(setting_value: &mut Option<i32>, new_setting_value: Option<i32>){
    match(setting_value.clone(), new_setting_value){        // 一般不会在 match 里面改掉引用所指向的内容, match 仅作模式匹配，可以用 setting_value.is_some() 进行判断
        (Some(_), Some(_)) => {
            println!("Can't change already customized value");
        },
        _ => {
            *setting_value = new_setting_value;
            println!("setting_value = {:?}", setting_value);
        },
    }
}

// 使用嵌套的 _ 忽略部分值，(Some(_), Some(_)) => {}
fn using_nested_ignoring_partial_value(){
    let mut old_value1 = Some(1i32);
    let mut old_value2 : Option<i32> = None;

    let new_value1 = Some(2i32);
    let new_value2 = Some(4i32);

    customized_change(& mut old_value1, new_value1);
    customized_change(& mut old_value2, new_value2);
}

fn print_partial_tuples(numbers: &[i32; 5]){
    match numbers{
        [first, _, third, _, fifth] => {
            println!("Some numbers is: {}, {}, {}", first, third, fifth);
        },
    }
}

// 使用嵌套忽略数组中的部分值
fn using_nested_ignoring_match_tuple(){
    let numbers1 : [i32; 5] = [1, 2, 3, 4, 5];
    let numbers2 : [i32; 5] = [2, 4, 6, 8, 10];

    print_partial_tuples(&numbers1);
    print_partial_tuples(&numbers2);
}

fn _vs_x_difference(){
    let s = Some(String::from("Hello, Rust"));

    if let Some(_s) = s{
        println!("Found a string");
    }

    // println!("value of s : {:?}", s);       // 产生报错， _s 会将值绑定到变量，s的值会被转移给 _s, 在 println! 中再次使用 s 会报错

    let x = Some(String::from("Hello, Rust again"));
    if let Some(_) = x{
        println!("Found a string");
    }

    println!("value of x : {:?}", x);
}


#[warn(dead_code)]
struct Point{
    x: i32,
    _y: i32,
    _z: i32,
}

fn only_print_partial_point(point: &Point){
    match point{
        Point{x, ..} => {
            println!("the value of x is {}", x);
        },
    }
}

fn ignoring_struct(){
    let point1 = Point{x: 1, _y: 1, _z: 2};
    let point2 = Point{x: 0, _y: 2, _z: 3};

    only_print_partial_point(&point1);
    only_print_partial_point(&point2);
}

fn only_print_partial_tuple(numbers: &(i32, i32, i32, i32, i32)){   // 元组类型的显式声明，标注出每一个成员的具体类型
    match numbers{
        (first, .., last) => {
            println!("first is {}, last is {}", first, last);
        }
    }
}

fn ignoring_tuple(){
    let numbers = (1, 2, 3, 4, 5);
    only_print_partial_tuple(&numbers);
}

fn match_guard_example(){
    // 1. 匹配守卫判断数
    let x = Some(5);
    match x {
        Some(n) if n > 10 => {
            println!("greater than 10, n is {:?}", n);
        },
        Some(n) => {
            println!("less than 10, n is {:?}", n);
        },
        _ => println!("other case"),
    }

    // 2. 匹配守卫，使用不同名的变量，不会覆盖外部的值
    let some_value_x = Some(5);
    let y = 10;

    match some_value_x{
        Some(n) if n == y =>  {
            println!("matched! n is {:?}", n);
        },
        Some(n) => {
            println!("failed to match! n is {:?}", n);
        },
        _ => println!("other case"),
    }

    println!("some_value_x :{:?}, y: {:?}", some_value_x, y);

    let bool_y = false;
    let value_x = 6;
    match value_x {
        4 | 5 | 6 if bool_y => {
            println!(" x matches");
        },
        _ => println!("other case"),
    }
}

enum Message{
    Hello{id: i32},
}

// 第一种绑定的用法：对结构体类型中，满足某一范围值的变量进行绑定
fn at_example1(){
    let msg = Message::Hello{id: 5};

    match msg{
        Message::Hello{id: id_variable @ 3..= 7} => {
            println!("id_variable: {}", id_variable);
        },
        Message::Hello{id: 10} => {
            println!("id equals 10");
        },
        Message::Hello{id} => println!("No binding, {}", id),
    }
}

#[derive(Debug)]
struct AtPoint{
    x: i32,
    y: i32,
}

// 第二种绑定的用法：对结构体类型进行绑定
#[warn(unused_variables)]
fn at_example2(){
    let point = AtPoint{x: 10, y: 32};
    let p @AtPoint{x: px, y: py} = AtPoint{x: 3, y : 23};
    println!("px = {}, py = {}", px, py);
    println!("AtPoint struct: {:?}", p);

    if let p@AtPoint{x: 10, y} = point{
        println!("y = {}", y);
        println!("{:?}", p);
    }
    else{
        println!("Can't match!");
    }
}

fn at_example3(){
    let x = 1;
    match x {
        num @ (1 | 2) => {
            println!("num = {}", num);
        },
        _ => println!("Not 1 or 2"),
    }
}

/*
练习 6 遇到的问题
&mut String 在 match 语句的匹配中，被当作 String 本身使用
*/
fn exercise_6(){
    let mut value = String::from("Hello");
    let r = &mut value;

    match r {
        s => {
            s.push_str(" Rust");
        },
    }

    println!("value = {}", value);
}

fn main(){
    using_nested_ignoring_partial_value();      // 使用嵌套的 _ 忽略部分值
    using_nested_ignoring_match_tuple();        // 使用嵌套的 _ 打印部分元组

    _vs_x_difference();                         // 使用 s 和 _s 比较差别
    ignoring_struct();                          // 忽略结构体中的值
    ignoring_tuple();                           // 忽略元组中的值  

    match_guard_example();                      // 匹配守卫的例子  

    at_example1();                              // @ 使用用例，进行带范围的变量绑定, 枚举结构体类型中，对字段进行变量绑定
    at_example2();                              // @ 绑定结构体，解构其中的 x, y, 整体打印
    at_example3();                              // @ 绑定单独的变量范围

    exercise_6();                               // 练习的例子
}