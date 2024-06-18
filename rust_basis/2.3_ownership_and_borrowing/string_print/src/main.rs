/*
熟悉 rust 中 String 的使用:
1. &str 硬编码的字符串字面量，在编代码的时候就知道字符串的值
let s = "hello";
2. String 类型，分配在堆上，可以动态调整字符串的大小
*/
fn string_related(){
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}",s);
}

/*
所有权转移:
对于 String 等堆区分配的数据，
如果是浅拷贝（拷贝 3 个 8字节大小的变量，分别是: 指向堆区的指针，当前占用的长度，可用的字节空间大小)
如果是深拷贝(需要在堆上重新分配内存空间)

浅拷贝会导致二次释放的问题
每个变量在离开作用域范围时，rust会调用 drop 释放堆区的内存
所有权转移: 当 s1 的值赋值给 s2 之后，不能再通过 s1 调用对应的 String
*/
fn ownership_change(){
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}, world", s1); // error: value borrowed here after move
    println!("{}, world", s2);
}

/*
继续引用 x 并没有报错
String 中 s1 持有了通过 String::from("hello") 创建的值的所有权
x 只是引用了存储在二进制中的字符串 "hello,world", 并没有持有所有权
*/
fn borrow_ownership(){
    let x: &str = "hello";
    let y = x;
    println!("{}, {}", x, y);
}

/*
所有权规则:
1. Rust 中每一个值都会被一个变量所拥有，该变量被称为值的所有者
2. 一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者
3. 当所有者（变量）离开作用域范围时，这个值将被丢弃(drop)
*/
fn main(){
    string_related();
    ownership_change();

    borrow_ownership();
}