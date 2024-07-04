/*
实现 Web事件的检查机制
*/
enum WebEvent{
    // Unit-like
    PageLoad,
    PageUnload,

    // tuple-struct
    Press(char),
    Paste(String),

    // c-like struct
    Click{
        x: i32,
        y: i32,
    },
}

fn inspect(event: WebEvent){
    match event {
        WebEvent::PageLoad => println!("PageLoad"),
        WebEvent::PageUnload => println!("PageUnload"),
        WebEvent::Press(c) => println!("Press at character \"{}\".", c),
        WebEvent::Paste(s) => println!("Paste information: {}", s),
        WebEvent::Click{x, y} => {
            println!("Click at x:{}, y:{}", x, y);
        },
    }
}

fn main(){
    let pageload = WebEvent::PageLoad;
    let pageunload = WebEvent::PageUnload;

    let press = WebEvent::Press('c'); 
    let paste = WebEvent::Paste("Sending this message".to_owned()); // 转移所有权类型, 从 &str -> String

    let click = WebEvent::Click{x: 20, y: 30};

    inspect(pageload);
    inspect(pageunload);
    inspect(press);
    inspect(paste);
    inspect(click);
}