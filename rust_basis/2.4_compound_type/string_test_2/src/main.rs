/*
由于习题页: 2.4.1 字符串与切片 String 部分 404 了，现在根据习题解答来完成
*/
/*
猜想: 这里原来写成 move_owership(s);
    之后 assert_eq!(s, "hello, world!"); 报错，因为 move_ownership(s: String) 传入的参数是 String 类型，
    s 的所有权在调用 move_ownership 之后被函数持有，函数结束后释放，之后再次调用 s 时，导致失去所有权的变量（已释放）再次被索引，出错
*/
fn exercise_1_1(){
    let mut s = String::from("hello, ");
    s.push_str("world");
    s.push('!');

    move_ownership(s.clone());
    assert_eq!(s, "hello, world!");
}

fn move_ownership(s: String){
    println!("ownership of \"{}\" is moved here!", s);
}

/*
这里传递的是引用 &str, 不会导致所有权发生变化。
*/
fn exercise_1_2(){
    let mut s: String = String::from("hello,");
    s.push_str("world");
    s.push('!');

    borrow_string(&s);

    assert_eq!(s, "hello,world!");
}

fn borrow_string(s: &str) {
    println!("ownership of \"{}\" is still with the variable 's', only the reference is passed", s);
}

/*
这里对 push 方法进行了一个补充说明
push 需要操作 String 或者 &mut String, 如果直接使用 &s, 是 &str 类型的自动转换，需要加上 mut
*/
fn exercise_2(){
    let mut s = String::from("hello, world");

    let slice1: &str = s.as_str();
    assert_eq!(slice1, "hello, world");

    let slice2 = &s[0..5];
    assert_eq!(slice2, "hello");

    // The type here cant be `&mut str` due to `push` is only defined on String type and its mut reference: `&mut String`!
    // can't use `s.as_mut_str()`
    let slice3:&mut String = &mut s;
    slice3.push('!');
    assert_eq!(slice3, "hello, world!");
}

/*
这里 for (i, c) in s.chars().enumerate() 是按照字符进行索引
*/
fn exercise_3(){
    let s = String::from("hello, 世界");
    let slice1 = &s[0..1];
    assert_eq!(slice1, "h");

    let slice2 = &s[7..10];
    assert_eq!(slice2, "世");

    for (i, c) in s.chars().enumerate() {
        if i == 7 {
            assert_eq!(c, '世');
        }
    }
}

/*
将字节向量 Vec<u8> 转换为字符串 String
其中 String::from_utf8 返回一个 Result<String, utf8Error> 类型的结果
unwrap() 方法从 Result<String, utf8Error> 类型中读取 String 结果
*/
fn exercise_4(){
   let mut s = String::new();
   s.push_str("hello");

   let v = vec![104, 101, 108, 108, 111];
   let s1 = String::from_utf8(v).unwrap();
   println!("s1 = {}", s1);

   assert_eq!(s, s1);
}

/*
String 的 with_capacity() 方法，对字符串预分配一定大小的容量，防止字符串反复申请，减少内存碎片，提高内存分配的效率
当超过容量时，会自动扩容
这个扩容的容量，有一定的算法在
*/
fn exercise_5(){
    let mut s = String::with_capacity(25);

    println!("s.len() = {}, s.capacity() = {}", s.len(), s.capacity());

    for _ in 0..5 {
        s.push_str("hello world!");
        println!("s.len() = {}, s.capacity() = {}", s.len(), s.capacity());
    }

    println!("{}", s);
}

/*
关于生命周期和 String 的一些思考
*/
// 导入 std::mem 模块，它包含了内存相关的实用工具，如 ManuallyDrop
use std::mem;

fn exercise_6(){
    let story = String::from("Rust by Practice");

    // 使用 ManuallyDrop 包装 story, 这阻止了 Rust 的所有权系统自动释放 story 的资源。
    // ManuallyDrop 允许我们手动控制资源的释放时间。
    let mut story = mem::ManuallyDrop::new(story);

    // 获取 story 字符串内部数据的可变指针, 返回字符串内部数据的原始指针
    let ptr = story.as_mut_ptr();
    // 获取字符串的长度，指字节数
    let len = story.len();
    // 获取 story 能存储的字节数
    let capacity = story.capacity();

    assert_eq!(16, len);

    /*
    使用 String::from_raw_parts 构造函数，根据原始指针，长度和容量重建一个 String 对象。
    这是不安全的代码，因为 Rust 无法检验这些组件的有效性，需要使用 unsafe 块
    最后检验原 story 字符串的内容与通过 from_raw_parts 重建的 String 对象 s 的内容是否相等
    */
    let s = unsafe{String::from_raw_parts(ptr, len, capacity)};

    assert_eq!(*story, s);
}

fn main(){
    exercise_1_1();
    exercise_1_2();

    exercise_2();
    exercise_3();
    exercise_4();

    exercise_5();

    exercise_6();
}