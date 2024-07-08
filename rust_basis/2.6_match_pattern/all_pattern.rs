// 匹配字面值
fn literal_match(){
    let one = 1;
    match one {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("Anything"),
    }
}

// 匹配命名变量
fn named_variable_match(){
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Got y is {:?}", y),
        _ => println!("Default: x is {:?}", x),
    }

    println!("x is {:?}, y is {:?}", x, y);
}

// 单分支多模式
fn single_branch_multi_mode(){
    let one = 1;
    match one {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("Anything else"),
    }
}

// 通过序列 ..= 进行匹配
fn match_range_character_or_number(){
    let x = 5;
    match x {
        1 ..=5 => println!("in number from 1 .. 5"),
        _ => println!("Anything else"), 
    }

    let y = 'x';
    match y {
        'a'..='k' => println!("In the earlier ASCII"),
        'l'..='z' => println!("In the latter ASCII"),
        _ => println!("other character case"),
    }
}

struct Point{
    x: i32,
    y: i32,
}
// 解构结构体
fn destruct_structure(){
    let p = Point{x: 0, y : 1};

    let Point{x: a, y: b} = p;
    assert_eq!(a, 0);
    assert_eq!(b, 1);

    let Point{x, y} = p;        // 这里可以这么做，是因为变量名与结构名一致
    assert_eq!(x, 0);
    assert_eq!(y, 1);

    // 使用模式匹配的方式，解构固定字面量值的结构体
    let point = Point{x: 0, y: 7};
    match point {
        Point{x , y: 0} => println!("the x axis is {}", x),
        Point{x: 0, y} => println!("the y axis is {}", y),
        Point{x, y} => println!("On the axis is ({}, {})", x, y),
    }
}

enum Message{
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 解构枚举
fn destruct_enum(){
    let message_list = [Message::ChangeColor(0, 160, 255), Message::Quit,
                Message::Write("Hello rust".to_string()), Message::Move{x: 0, y: 7}];
    for message in message_list{
        match_message(message);
    }
}

fn match_message(message: Message){
    match message {
        Message::Quit => println!("Message Quit"),
        Message::Move{x , y} => {
            println!("Move from (0,0) to ({}, {})", x, y);
        },
        Message::Write(s) => {
            println!("Message info {}", s);
        },
        Message::ChangeColor(r, g, b) => println!("The color red: {}, green: {}, blue: {}", r, g, b),
    };
}

enum Color{
    Rgb{
        r: i32,
        g: i32,
        b: i32,    
    },
    Hsv{
        h: i32,
        s: i32,
        v: i32,
    },
}

enum ColorMessage{
    Quit,
    Write(String),
    MoveTo(Point),
    ChangeColor(Color),
}

fn destruct_nested_enum(){
    println!("constructing colorMessage");
    let color_message_list = [
        ColorMessage::Quit,
        ColorMessage::Write("Hello Rust-By-Practice".to_string()),
        ColorMessage::MoveTo(Point{x: 16, y: 23}),
        ColorMessage::ChangeColor(Color::Rgb{r: 0, g: 0, b: 255}),
        ColorMessage::ChangeColor(Color::Hsv{h: 1, s: 0, v: 2}),
    ];

    println!("destructing colorMessage");
    
    for msg in color_message_list{
        read_colormessage(msg);
    }
}

// 解构嵌套的枚举例子
fn read_colormessage(message: ColorMessage){
    match message{
        ColorMessage::Quit => println!("Quit"),
        ColorMessage::Write(s) => println!("Write {}", s),
        ColorMessage::MoveTo(Point{x, y}) => println!("MoveTo ({}, {})", x, y),
        ColorMessage::ChangeColor(Color::Rgb{r, g, b}) => {
            println!("RGB format! red:{}, green:{}, blue:{}", r, g, b);
        },
        ColorMessage::ChangeColor(Color::Hsv{h, s, v}) => {
            println!("Hsv format! hue:{}, saturation:{}, value:{}", h, s, v);
        },
    }
}

// 解构元组和结构体
fn destruct_tuple_and_structure(){
    let ((feet, inches), Point{x, y}) = ((3, 10), Point{x: 30, y : -9});
    println!("{}, {}, x = {}, y = {}", feet, inches, x, y);
}

// 解构数组
fn destruct_array(){
    // 定长数组
    println!("fixed length array");
    let fixed_array : &[u16; 2] = &[114, 514];
    let &[fixed_x, fixed_y] = fixed_array;
    println!("fixed_x: {}, fixed_y: {}", fixed_x, fixed_y);

    // 不定长数组
    println!("Non-fixed length array");
    let unfixed_array : &[u16] = &[114, 514];

    if let [x, ..] = unfixed_array{
        println!("x is {:?}", x);
    }

    if let &[.., y] = unfixed_array{
        println!("y is {:?}", y);
    }

    let non_array : &[i32; 0]= &[];
    assert!(matches!(non_array, &[..]));
    assert!(matches!(non_array, &[]));
    // assert!(!matches!(non_array, &[x, ..]));  // 似乎编译期就能检查出错误
    assert!(matches!(unfixed_array, &[x, ..]));
    assert!(matches!(unfixed_array, &[.., y]));
}

/* 全部模式匹配的情况，工具书章节 */
fn main(){
    literal_match();                            // 匹配字面值
    named_variable_match();                     // 匹配命名变量

    single_branch_multi_mode();                 // 单分支多模式
    match_range_character_or_number();          // 使用 range ..= 进行模式匹配

    destruct_structure();                       // 解构结构体
    destruct_enum();                            // 解构枚举
    destruct_nested_enum();                     // 解构嵌套枚举

    destruct_tuple_and_structure();             // 解构元组和结构体
    destruct_array();                           // 解构数组
}