# 流程控制

## if 分支控制
```rust
fn main(){
    let condition = true;
    let number = if condition {
        5
    }else{
        6
    };

    println!("The value of number is :{}", number);
}
```
if 语句块是表达式，这里我们使用 if 表达式的返回值来给 number 进行赋值: number 的值是 5
用 if 来赋值时，要保证每个分支返回的类型一样（事实上，这种说法不完全准确），此处返回的 5 和 6 就是同一个类型，如果返回类型不一致就会报错。

## 使用 else if 来处理多重条件
可以将 else if 与 if, else 组合在一起实现更复杂的条件分支判断:
```rust
fn main(){
    let n = 6;

    if n % 4 == 0 {
        println!("number is divisible by 4");
    }else if n % 3 == 0 {
        println!("number is divisible by 3");
    }else if n % 2 == 0 {
        println!("number is divisible by 2");
    }else{
        println!("number is not divisible by 4, 3 or 2");
    }
}
```
## 循环
Rust 语言中有三种循环方式, for , while 和 loop
### for 循环
```rust
fn main(){
    for i in 1..=5 {
        println!("{}", i);
    }
}
```
核心在于 for 和 in 的联动，语义表达如下:
```rust
for 元素 in 集合 {
    // 使用元素干一些你懂我不懂的事情
}
```
使用 for 时我们往往使用集合的引用形式，除非你不想在后面的代码中继续使用该集合（比如我们这里使用了 container 的引用）。
如果不使用引用的话，所有权会被转移(move) 到 for 语句块中，后面就无法再使用这个集合了:
```rust
for item in &container {
    // ...
}
```
对于实现了 copy 特征的数组(例如 [i32; 10]) 而言，for item in arr 并不会把 arr 的所有权转移，而是直接对其进行了拷贝，因此循环之后仍然可以使用 arr。
如果想在循环中，修改该元素，可以使用 mut 关键字:
```rust
for item in &mut collection {

}
```
总结如下:
使用方法:
1. for item in collection 
等价使用方式 for item in IntoIterator::into_iter(collection) 
转移所有权
2. for item in &collection
等价使用方式 for item in collection.iter() 不可变借用
3. for item in &mut collection
等价使用方式 for item in collection.iter_mut() 可变借用

如果想在循环中获取元素的索引:
```rust
fn main(){
    let a = [4, 3, 2, 1];
    // `.iter()` 方法把 `a` 数组变成一个迭代器
    for (i, v) in a.iter().enumerate() {
        println!("第 {} 个元素是 {}", i + 1, v);
    }
}
```

如果我们想用 for 循环控制某个过程执行 10 次，但是又不想单独声明一个变量来控制这个流程，该怎么写?
```rust
for _ in 0..10 {
    // ...
}
```
可以用 _ 来替代 i 用于 for 循环中, 在 Rust 中 _ 的含义是忽略该值或类型的意思，如果不使用 _, 那么编译器会给一个变量未使用的警告。

#### 两种循环方式优劣对比
```rust
// 第一种
let collection = [1,2,3,4,5];
for i in 0..collection.len() {
    let item = collection[i];
    // ...
}

// 第二种
for item in collection {

}
```

- 性能
第一种使用方式中 collection[index] 的索引访问，会因为边界检查(Bounds checking) 导致运行时的性能损耗，
Rust 会检查并确认 index 是否落在集合内， 
第二种直接迭代的方式就不会触发这种检查，因为编译器会在编译时就完成分析并证明这种访问是合法的。
- 安全
第一种方式里对 collection 的索引访问是非连续的，存在一定可能性在两次访问之间， collection 发生了变化，导致脏数据产生。
第二种直接迭代的方式是连续访问，不存在这种风险（由于所有权限制，在访问过程中，数据并不会发生变化）。

for 循环无需任何条件限制，也不需要通过索引来访问，因此是最安全也最常用的。

## continue
```rust
for i in 1..4 {
    if i == 2 {
        continue;
    }
    println!("{}", i);
}
```
break 是跳出整个

## while 循环
```rust
fn main(){
    let mut n = 0;

    while n <= 5 {
        println!("{}!", n);
        n = n + 1;
    }
    println!("我出来了!");
}

fn main(){
    let mut n = 0;
    loop {
        if n > 5 {
            break;
        }
        println!("{}", n);
        n += 1;
    }
    println!("我出来了!");
}
```

### while vs for
```rust
fn main(){
    let a = [10,20,30,40,50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);
        index = index + 1;
    }
    // for 的写法
    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```
## loop 循环
使用 loop 时，必不可少的是 break 关键字, 它能让循环在某个条件时跳出:
```rust
fn main(){
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
}
```
当 counter 递增到 10 时，就会通过 break 返回一个 count * 2 的值，最后赋值给 result 并打印出来。

1. break 可以单独使用，也可以带一个返回值，类似 return
2. loop 是一个表达式，因此可以返回一个值