# 模式适用场景
模式
模式是 Rust 中的特殊语法，它用来匹配类型中的结构和数据，往往和 match 表达式联用，以实现强大的模式匹配能力。
- 字面值
- 解构的数组、枚举、结构体或者元组
- 变量
- 通配符
- 占位符
  
## 所有可能用到模式的地方
#### match 分支
```rust
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}
```
match 的每个分支就是一个模式，match 匹配是穷尽式的，因此我们往往需要一个特殊的模式 _ 来匹配剩余的所有情况:
```rust
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    _ => EXPRESSION,
}
```

#### if let 分支
if let 往往用于匹配一个模式，而忽略剩下的所有模式的场景:
```rust
if let PATTERN = SOME_VALUE {

}
```

#### while let 条件循环
一个与 if let 类似的结构是 while let 条件循环，它允许只要模式匹配就一直进行 while 循环。下面展示了一个使用 while let 的例子:
```rust
// Vec 是动态数组
let mut stack = Vec::new();

// 向数组尾部插入元素
stack.push(1);
stack.push(2);
stack.push(3);

// stack.pop 从数组尾部弹出元素
while let Some(top) = stack.pop() {
    println!("{}", top);
}
```
打印出 3, 2, 1
pop 方法取出动态数组的最后一个元素并返回 Some(value), 如果动态数组是空的，将返回 None，
对于 while 来说，只要 pop 返回 Some 就会一直不停地循环。
一旦其返回 None, while 循环停止。我们可以使用 while let 来弹出栈中的每一个元素。

也可以使用 loop + if let 或者 match 来实现这个功能，但是会更加啰嗦。

### for 循环
```rust
let v = vec!['a', 'b', 'c'];

for (index, value) in v.iter().enumerate(){
    println!("{} is at index {}", value, index);
}
```

### let 语句
let x = 5;
x 也是一种模式绑定，代表将匹配的值绑定到变量 x 上。因此，在 Rust 中，变量名也是一种模式。
```rust
let (x, y, z) = (1, 2, 3);
```
对元组来说，元素个数也是类型的一部分！

### 函数参数
函数参数也是模式:
```rust
fn foo(x: i32){
    // 代码
}
```
其中 x 就是一个模式，还可以在参数中匹配元组:
```rust
fn print_coordinates(&(x, y): &(i32, i32)){
    println!("Current location: ({}, {})", x, y);
}

fn main(){
    let point = (3, 5);
    print_coordinates(&point);
}
```
#### let 和 if let
对于以下代码，编译器会报错:
```rust
let Some(x) = some_option_value;
```
右边的值可能不为 Some, 而是 None, 这种时候就不能进行匹配，上面的代码遗漏了 None 的匹配。
类似 let, for, match 都必须要求完全覆盖匹配，才能通过编译(不可驳模式匹配)。

对于 if let, 就可以这样使用:
```rust
if let Some(x) = some_option_value {
    println!("{}", x);
}
```
可驳模式匹配。