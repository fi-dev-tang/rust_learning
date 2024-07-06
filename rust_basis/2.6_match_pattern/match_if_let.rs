#![allow(dead_code)]
#[derive(Debug)]
enum Direction{
    South,
    North,
    East,
    West,
}

fn simple_match_case(){
    let dire = Direction::South;
    match dire {
        Direction::West => println!("West direction"),
        Direction::North | Direction::South => println!("North or South direction"),
        // 如果不写 _ => println!("East direction"), 会产生报错: pattern `Direction::East` not covered
        _ => println!("East direction"),
    };
}

// 模式执行的返回值必须是同一类型的表达式，用 , 分隔
enum Coin{
    Penney,
    Nickel,
    Dime,
    Quarter,
}

fn match_coin_case(){
    let coin_1 = Coin::Penney;
    let coin_2 = Coin::Nickel;
    let coin_3 = Coin::Dime;
    let coin_4 = Coin::Quarter;

    println!("{}\n{}\n{}\n{}", measure_coin(coin_1), measure_coin(coin_2), measure_coin(coin_3), measure_coin(coin_4));
}

fn measure_coin(coin: Coin) -> u8{
    match coin {
        Coin::Penney => {
            println!("You are lucky, this is lucky penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 2,
        Coin::Quarter => 10,
    }
    // 不能加分号，否则 match 语句执行完之后没有返回值
}

#[allow(dead_code)]
enum IpAddr{
    Ipv4,
    Ipv6,
}

fn match_as_return_result(){
    let ip_addr = IpAddr::Ipv4;
    let _ipv6_addr = IpAddr::Ipv6;

    let ip_str = match ip_addr{
        IpAddr::Ipv4 => "127.0.0.1",
        _ => "::1",
    };
    println!("{}", ip_str);
}

// 从模式匹配中获取值
#[derive(Debug)]
enum UsState{
    Alaska,
    Aston,
}

enum CoinSt{
    Nickel,
    Dime, 
    Penney,
    Quarter(UsState),
}

fn get_value_from_match(){
    let coinst = CoinSt::Quarter(UsState::Alaska);
    match coinst {
        CoinSt::Nickel => println!("Nickel, value = 1"),
        CoinSt::Dime => println!("Dime, value = 2"),
        CoinSt::Penney => println!("Penny, value = 5"),
        CoinSt::Quarter(state) => {
            println!("The value of state is {:?}!", state)
        },      
        // 在匹配 Coin::Quarter(state) 模式时，我们把它内部存储的值绑定到了 state 变量上，
        // 因此 state 变量就是对应的 UsState 枚举类型
    };
}

/*
更复杂的模式匹配拆解变量的例子
*/
enum Action{
    Say(String),
    MoveTo(u16, u16),
    ChangeColorRGB(i32, i32, i32),
}

fn more_complex_pattern_destruct(){
    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(0, 1),
        Action::ChangeColorRGB(255, 255, 0)
    ];

    for action in actions {
        match action {
            Action::Say(string) => {
                println!("Action::Say {}", string);
            },
            Action::MoveTo(x, y) => {
                println!("Action::MoveTo({}, {})", x, y);
            },
            Action::ChangeColorRGB(r,g, _) => {
                println!("Action::ChangeColorRGB({},{},...)", r, g);
            },
        };
    }
}

/*
大多数情况下，当我们并不关心某些数值时，使用 _ 
或者 other 处理其他多余的情况
*/
fn wirdcard_character(){
    let some_value: u8 = 0u8;
    match some_value {
        0 => println!("value is 0"),
        1 => println!("vaule is 1"),
        3 => println!("value is 3"),
        5 => println!("value is 5"),
        7 => println!("value is 7"),
        _ => ()
    };

    let dire = Direction::East;
    match dire{
        Direction::East => println!("direction is east"),
        other => println!("we don't care the direction {:?}", other),
    };
}

/*
比较 if let 和 match 的使用场景
由于 Rust 编译器强制要求 match 覆盖所有的分支场景，
当我们只需要处理多分支中的一种值，其他的值不需要考虑时，使用 if let
*/
fn if_let_vs_match(){
    let value_1 = Some(3u8);

    match value_1 {
        Some(3u8) => println!("three"),
        _ => (),
    };

    if let Some(3u8) = value_1 {
        println!("if let: three");
    }
}

/*
重要的区别点:
matches! 宏的使用场景
对于数组等枚举类型进行过滤时，可以完成批量过滤的条件, matches! 直接返回 true 或者 false
*/
#[derive(Debug)]
enum MyEnum{
    Foo,
    Bar,
}

/*
这种写法有问题:
myenum_list.iter().filter(|x| matches!(x, MyEnum::Foo));
*/
fn matches_example(){
    /*
    let myenum_list = [MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    myenum_list.iter().filter(|x| matches!(x, MyEnum::Foo));
    println!("myenum_list = {:?}", myenum_list);
    
    对原来写法修改的注释:
    在 Rust 中，iter() 和 filter() 都不会修改原集合，而是返回一个新的迭代器，该迭代器包含了经过过滤之后的元素序列。
    当你调用 myenum_list.iter().filter(|x| matches!(x, MyEnum::Foo)); 时
    实际创建了一个新的迭代器，其中包含了所有匹配 MyEnum::Foo 的项，
    但这个迭代器并没有被存储或转换回集合类型。

    如果你想要看到过滤后的结果，你需要收集(collect) 这些迭代器产生的值到一个新的集合中，collect()
    可以写成引用 Vec<&MyEnum> 或者自动类型推断 Vec<_> 
    */
    let myenum_list = [MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    let filter_list: Vec<&MyEnum> = myenum_list.iter().filter(|x| matches!(x, MyEnum::Foo)).collect();
    println!("before myenum_list: {:?}", myenum_list);
    println!("filter_list: {:?}", filter_list);
}

fn main(){
    simple_match_case();                // 一个简单的 match 匹配实例
    match_coin_case();                  // match 执行的表达式中具有语句，和返回的表达式
    match_as_return_result();           // match 模式匹配作为返回值

    get_value_from_match();             // 从 match 模式匹配中获取 state 的值，例如 Coin::Quarter(UsState::Australia)
    more_complex_pattern_destruct();    // 更复杂的模式匹配的解构和拆分

    wirdcard_character();               // 通配符 _ 或者 other 匹配其他多余的情况
    if_let_vs_match();                  // if let 判断单条件分支与 match 的比较

    matches_example();                  // 使用 matches! 宏批量过滤枚举类型数组等
}