/*
第一种: 粗糙实现扑克牌的枚举类型，和扑克牌结构体
*/
#[derive(Debug)]
enum PokerSuit_1 {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

#[derive(Debug)]
struct PokerCard_1 {
    suit: PokerSuit_1,
    value: u8,
}

// 粗糙实现 pokercard,其中 suit 字段是枚举类型，对应扑克牌 4 种花色的枚举值
fn coarse_implement_pokercard(){
    let c1 = PokerCard_1{
        suit: PokerSuit_1::Diamonds,
        value: 8,
    };

    let c2 = PokerCard_1{
        suit: PokerSuit_1::Spades,
        value: 12,
    };

    print_card1(c1);
    print_card1(c2);
}

fn print_card1(card: PokerCard_1){
    println!("{:#?}", card);
}

/*
第二种实现扑克牌信息的方式，直接将数据信息关联到枚举类型上
*/
#[derive(Debug)]
enum PokerCard_2{
    Diamonds(u8),
    Hearts(u8),
    Clubs(char),
    Spades(char),
}

fn nice_implement_pokercard(){
    let c1 = PokerCard_2::Diamonds(5);
    let c2 = PokerCard_2::Hearts(3);
    let c3 = PokerCard_2::Clubs('K');
    let c4 = PokerCard_2::Spades('Q');

    print_card2(c1);
    print_card2(c2);
    print_card2(c3);
    print_card2(c4);
}

fn print_card2(card: PokerCard_2){
    println!("{:#?}", card);
}

#[derive(Debug)]
enum Message{
    Quit,
    Move {x: i32, y: i32},
    Color(i32, i32, i32),
    Write(String),
}

/*
枚举类型中可以含有不同种类的成员，也可以包含结构体等
如果不使用枚举类型，全部写成结构体

struct QuitMessage;     // 单元结构体
struct MoveMessage{
    x: i32,
    y: i32,
}                       // 普通结构体

struct Color(i32, i32, i32); // 元组结构体
struct Write(String);        // 元组结构体
类型不能做到统一
*/
fn create_message(){
    let m1 = Message::Quit;
    /*
    语法错误，
    不能写成 Message::Move(2,3), 这里的 Move 是普通结构体
    Message::Move{x: 2, y: 3};
    */
    let m2 = Message::Move{x: 2, y: 3};
    let m3 = Message::Color(0,0,0);
    let m4 = Message::Write("Hello rust".to_string());
    
    print_message(m1);
    print_message(m2);
    print_message(m3);
    print_message(m4);
}

fn print_message(m: Message){
    println!("{:#?}", m);
}

/*
Rust 引入 Option 枚举类型来处理空值的情况，其它情况下默认不为空值
对空值需要做特殊处理:
enum Option<T> {
    Some(T),
    None,
}

原理: 对于滥用的 null 值进行设置，但凡 Option<T> 里面出现 None 值，都会抛出 match None => 让用户处理
*/
fn option_example(){
    let five: u32 = 5;
    let option_five: Option<u32> = Some(5);
    let option_six = Some(6);
    let option_none : Option<u32> = None;

    println!("{:?}", plus_one(option_six));
    println!("{:?}", plus_one(option_none));
}

fn plus_one(s: Option<u32>) -> Option<u32>{
    match s {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main(){
    coarse_implement_pokercard();       // 粗糙实现 PokerCard 类型
    nice_implement_pokercard();         // 直接将数据信息关联到枚举成员上
    create_message();                   // 枚举类型成员多样

    option_example();                   // Rust 引入 Option 枚举类型，对空值做特殊处理
}