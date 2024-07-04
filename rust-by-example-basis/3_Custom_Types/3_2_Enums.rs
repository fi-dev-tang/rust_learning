/*
Enums
The enum keyword allows the creation of a type which may be one of a few different variants.
Any variant which is valid as a struct is also valid in an enum.
*/
/* Create an `enum` to classify a web event. Note how both names and type information together specify the variant:
`PageLoad != PageUnload` and `KeyPress(char) != Paste(String).`
Each is different and independent.
*/
enum WebEvent{
    // An `enum` variant may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char), 
    Paste(String),
    // or c-like structures.
    Click {x: i64, y: i64},
}

fn inspect(event: WebEvent){
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum` variant.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click{x, y} => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

fn main(){
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    /*
    to_owned(): 从借用的 (&T) 类型转换为拥有所有权的 (T) 类型，也就是创建一个数据的副本。
    主要用在字符串(&str 转 String) 和切片(比如 &[T] 转 Vec<T> 上)
    1. let x: &str = "hello world"; let y = s.to_owned();
    2. let x = [1,2,3,4,5]; let y: &[i32] = &x; let z = y.to_owned(); 
    最后的 z 是一个 Vec<i32>, 包含了 slice 中的所有元素，与原数组独立开来，拥有自己的内存空间。
    */
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click{x: 20, y: 80};
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}