// 使用两种方法修复
/*
let s: Box<str> = "String Exercise 2".into();
此时 s 的类型是 str
Rust 语言中的一个类型表示，它是一个智能指针，指向在堆上分配的字符串数据。
"Box" 是 Rust 标准库提供的一个类型，用于在运行时动态分配数据并在堆上管理其内存。
<str> 表示这个 Box 存储的是 str 类型的数据。
*/
// & 可以将 Box<str> 转换为 &str 类型
fn exercise_2_way_1(){
    let s: Box<str> = "1: hello Rust".into();
    greets(&s);
}

fn exercise_2_way_2(){
   let s: Box<&str> = "2: hello Rust".into();
   greets(*s);
}

/*
let s: Box<str> = "hello world".into();
greets(&s);

let s: Box<&str> = "hello world".into();
greets(*s);

关于上面思路的两种解释: 
@source1:
当前面 s 是 Box<str> 类型，下面 greets 函数要传入的参数是 &str 类型，所以要使用 &s。
& 可以用来将 Box<str> 转换为 &str 类型，这样就可以正常传参了。

当前面 s 是 Box<&str> 类型，Box<T> 的解释: Box<T> 允许你将一个值分配到堆上，然后在栈上保留一个智能指针指向堆上的数据。
栈上保留的智能指针，指向的是 &str, 所以 *s 解引用，解的是智能指针的，让实际传入的参数仍然是 &str 类型
如果 let s : Box<str> 解引用解出来：*s 是 str 类型不匹配
*/
fn greets(s: &str){
    println!("{}", s);
}

/*
你只能将 String 跟 &str 类型进行拼接，并且 String 的所有权在此过程中会被 move
*/
fn exercise_6(){
    let s1 = String::from("Exercise 6");
    let s2 = String::from(" Hay!");
    let s3 = s1.clone() + &s2;

    /*
    由于 s1 调用了 fn add(self, &str),
    s1 的所有权被传递给 add, 在 add 结束之后，s1 被释放
    */
    assert_eq!(s3, "Exercise 6 Hay!");
    println!("{}", s1);
}

/* 
使用第三方库 utf8_slice 来访问 UTF-8 字符的某个字串
该库索引的是字符，而不是字节
*/
use utf8_slice;
fn using_utf8_slice(){
    let s = "The 🚀 goes to the 🌑!";
    println!("{}", s);

    let rocket = utf8_slice::slice(s, 4, 5);
    println!("{}", rocket);
}

fn main(){
    exercise_2_way_1();
    exercise_2_way_2();
    
    exercise_6();
    using_utf8_slice();
}