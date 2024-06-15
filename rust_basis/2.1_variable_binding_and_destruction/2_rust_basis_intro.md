# Rust 基本概念

之前没有接触过的概念:
- 所有权，借用，生命周期
- 宏编程
- 模式匹配
  
```rust
fn main(){
    let a = 10;
    let b: i32 = 20;

    let mut c = 30i32;
    let d = 30_i32;
    let e = add(add(a,b), add(c,d));

    println!("(a + b) + (c + d) = {}", e);
}

// 不要为 i + j 添加; 这会改变语法导致函数返回 () 而不是 i32
fn add(i: i32, j: i32) -> i32 {
    i + j
}
```