// 4. 使用枚举对类型进行同一化
// 填空，并修复错误
#[derive(Debug)]
enum Message{
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main(){
    let msgs = [  // 这里写成 let msgs: [Message, 3] 报错
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255, 255, 0)
    ];

    for msg in msgs {
        show_message(msg)
    }
}

fn show_message(msg: Message){
    println!("{:#?}", msg);
}