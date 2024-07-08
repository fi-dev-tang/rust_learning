fn option_match(){
    let five = Some(5);
    let six = plus_one(five);
    let option_none = plus_one(None);

    println!("six is {:?}, option_none is {:?}", six, option_none);
}

// Option<T> 类型的处理函数，用 Some 来描述
fn plus_one(x: Option<u32>) -> Option<u32>{
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
} 

fn main(){
    option_match();     // 实现 Option<T> 类型的处理函数
}