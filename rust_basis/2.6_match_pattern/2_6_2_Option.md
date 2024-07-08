# 解构 Option
Option 枚举，它用来解决 Rust 中变量是否有值的问题，
```rust
enum Option<T>{
    Some(T),
    None,
}
```
一个变量要么有值: Some(T), 要么为空: None。
> Option, Some, None 都包含在 prelude 中，因此可以直接通过名称来使用它们，无需以 Option::Some 这种形式去使用，
> 不要因为调用路径变短了，就忘记 Some 和 None 也是 Option 底下的枚举成员。

## 匹配 Option<T>
使用 Option<T>, 是为了从 Some 中取出其内部的 T 值以及处理没有值的情况，
编写一个函数，获取一个 Option<i32>, 如果其中含有一个值，将其加一，没有值，则函数返回 None 值。
```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```