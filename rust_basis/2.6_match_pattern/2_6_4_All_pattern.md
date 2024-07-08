# 全模式列表

## 匹配字面值
```rust
let x = 1;
match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

## 匹配命名变量
```rust
fn main(){
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end x = {:?}, y = {:?}", x, y);
}
```

## 单分支多模式
在 match 表达式中，可以使用 | 语法匹配多个模式，它代表 或的意思。例如，下面代码将 x 的值与匹配分支相比较，
```rust
let x = 1;
match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

## 序列匹配的情况
```rust
let x = 5;
match x {
    1..=5 => println!("one through five"),
    _ => println!("something else"),
}

let x = 'c';

match x {
    'a'..='j' => println!("eary ASCII letter"),
    'k'..='z' => println!("late ASCII letter"),
    _ => println!("something else"),
}
```
序列只允许数字或字符串，字符和数字值是 Rust 中仅有的可以用于判断是否为空的类型。
## 解构并分解值
使用模式来解构结构体、枚举、元组、数组和引用。

#### 解构结构体
用 let 解构一个带有两个字段 x 和 y 的结构体 Point:
```rust
struct Point{
    x: i32,
    y: i32,
}

fn main(){
    let p = Point{x: 0, y: 7};

    let Point {x: a, y: b} = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}
```
使用字面值作为结构体模式的一部分进行解构，而不是为所有的字段创建变量。这允许我们测试一些字段为特定值的同时创建其他字段的变量。
```rust
fn main(){
    let p = Point{x: 0, y: 7};

    match p {
        Point{x: 0, y} => println!("On the x axis at {}", x),
        Point{x, y: 0} => println!("On the y axis at {}", y),
        Point{x, y} => println!("On neither axis:({},{})", x, y),
    }
}
```
## 解构枚举
解构嵌套的结构体和枚举
目前为止，所有的例子都只匹配了深度为一级的结构体或枚举。match 也可以匹配嵌套的项!
例如使用下面的代码来同时支持 RGB 和 HSV 色彩模式:
```rust
enum Color{
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message{
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    ChangeColor(Color),
}

fn main(){
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation{}, and value {}",
                h,
                s,
                v
            )
        }
        _ => ()
    }
}
```
match 第一个分支的模式匹配一个 Message::ChangeColor 枚举成员，该枚举成员又包含了一个 Color::Rgb 的枚举成员，
最终绑定了 3 个内部的 i32 值。

## 解构结构体和元组
用复杂的方式混合、匹配和嵌套解构模式。
复杂结构体的例子，结构体和元组嵌套在元组中，并将所有的原始类型解构出来:
```rust
struct Point{
    x: i32,
    y: i32,
}

let((feet, inches), Point{x, y}) = ((3, 10), Point{x: 3, y: -10});
```

## 解构数组
类似元组的方式解构，分为两种情况:
定长数组
```rust
let arr: [u16; 2] = [114, 514];
let [x, y] = arr;

assert_eq!(x, 114);
assert_eq!(y, 514);
```

不定长数组:
```rust
let arr: &[u16] = &[114, 514];

if let [x, ..] = arr {
    assert_eq!(x, &114);
}

if let &[.., y] = arr{
    assert_eq!(y, 514);
}

let arr: &[u16] = &[];
assert!(matches!(arr, [..]));
assert!(!matches!(arr, [x, ..]));
```