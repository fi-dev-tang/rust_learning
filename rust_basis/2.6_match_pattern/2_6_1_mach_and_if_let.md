# 模式匹配
模式匹配这个词，对于非函数语言编程来说，比较少听到。
因为它经常出现在函数式编程里，用于为复杂的类型系统提供一个轻松的解构能力。

在枚举和流程控制那章，我们遗留了两个问题，都是关于 match 的，第一个是如何对 Option 枚举进行进一步处理，
另外一个是如何用 match 来替代 else if 这种丑陋的多重分支使用方式。

## match 和 if let
在 Rust 中，模式匹配最常用的就是 match 和 if let
一个关于 match 的简单例子:
```rust
enum Direction{
    East,
    West,
    North,
    South,
}

fn main(){
    let dire = Direction::South;
    match dire{
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        },
        _ => println!("West"),
    };
}
```
这里我们想去匹配 dire 对应的枚举类型，因此在 match 中用三个匹配分支来完全覆盖枚举变量 Direction 的所有成员类型
- match 的匹配必须要穷举出所有可能，因此这里用 _ 来代表未列出的所有可能性
- match 的每一个分支都必须是一个表达式，且所有分支的表达式最终返回值的类型必须相同
- X | Y , 类似逻辑运算符或，代表该分支可以匹配 X 也可以匹配 Y, 只要满足一个即可
  
match 跟其他语言中的 switch 非常像， _ 类似于 switch 中的 default。

## match 匹配
看看 match 匹配的通用形式:
```rust
match target {
    模式 1 => 表达式1,
    模式 2 => {
        语句1;
        语句2;
        表达式2
    },
    _ => 表达式3
}
```
将模式与 target 进行匹配，即为模式匹配，而模式匹配不仅仅局限于 match
match 允许我们将一个值与一系列的模式相比较，并根据相匹配的模式执行对应的代码
```rust
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```
value_in_cents 函数根据匹配到的硬币，返回对应的美分值。 match 后面紧跟的是一个表达式，跟 if 很像，
但是 if 后的表达式必须是个布尔值，而 match 后的表达式返回值可以是任意类型，只要能跟后面的分支中的模式匹配起来即可，这里的 coin 是枚举 Coin 类型。
如果分支有多行代码，那么需要用 {} 包裹，同时最后一行代码需要是一个表达式。

## 使用 match 表达式赋值
match 本身也是一个表达式，因此可以用它来赋值:
```rust
enum IpAddr{
    Ipv4,
    Ipv6
}

fn main(){
    let ip1 = IpAddr::Ipv6;
    let ip_str = match ip1 {
        IpAddr::Ipv4 => "127.0.0.1",
        _ => "::1",
    };

    println!("{}", ip_str);
}
```
因为这里匹配到 _ 分支，所以将 "::1" 赋值给了 ip_str。

### 模式绑定
模式匹配的另外一个重要功能是从模式中取出绑定的值，例如:
```rust
#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
```
希望在模式匹配的时候，获取到 25 美分硬币上刻印的州的名称:
```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
```
在匹配 Coin::Quater(state) 模式时，我们把它内部存储的值绑定到了 state 变量上，因此 state 变量就是对应的 UsState 枚举类型。
印了阿拉斯加州标记的 25 分硬币: Coin::Quarter(UsState::Alaska), 它在匹配时，state 变量将被绑定 UsState: Alaska 的枚举值。

一个更复杂的例子:
```rust
enum Action{
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

fn main(){
    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1, 2),
        Action::ChangeColorRGB(255, 255, 0),
    ];
    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{}", s);
            },
            Action::MoveTo(x, y) => {
                println!("point from (0,0) move to ({}, {})", x, y);
            },
            Action::ChangeColorRGB(r, g, _) => {
                println!("change color into '(r:{}, g:{}, b:0)', 'b' has been ignored", r, g);
            }
        }
    }
}
```

### 穷尽匹配
在文章的开头，我们简单总结过 match 的匹配必须穷尽所有情况，下面来举例说明，例如 _, other

### if let 匹配
有时会遇到只有一个模式的值需要处理，其它值直接忽略的场景，如果使用 match 来处理就要写成下面这样:
```rust
let v = Some(3u8);
match v {
    Some(3) => println!("three"),
    _ => (),
}
```
我们只想要对 Some(3) 模式进行匹配，不想处理任何其他 Some<u8> 值或 None 值。但是为了满足 match 表达式（穷尽性）的要求，
写代码时必须在处理完这唯一的成员后加上 _ => (), 这样会增加不少无用的代码。

完全可以用 if let 的方式实现:
```rust
if let Some(3) = v{
    println!("three");
}
```
**当你只要匹配一个条件，且忽略其他条件时就用 if let, 否则都用 match**

### matches! 宏
提供了一个非常实用的宏: matches!, 它可以将一个表达式跟模式进行匹配，然后返回匹配的结果 true or false。

有一个动态数组，里面存有以下枚举:
```rust
enum MyEnum{
    Foo,
    Bar
}

fn main(){
    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
}
```
现在如果想对 v 进行过滤，只保留类别是 MyEnum::Foo 的元素，你可能想这么写:
```rust
v.iter().filter(|x| x == MyEnum::Foo);
```
实际上这行代码会报错，因为你无法将 x 直接跟一个枚举成员进行比较。好在，你可以使用 match 来完成，但是会导致代码更为啰嗦，
是否有更简洁的方式？答案是使用 matches!:
```rust
v.iter().filter(|x| matches!(x, MyEnum::Foo));
```
很简单也很简洁，看更多的例子
```rust
let foo = 'f';
assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));

let bar = Some(4);
assert!(matches!(bar, Some(x) if x > 2));
```