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
Rust 中不会自动进行复制的“深拷贝”，但是可以调用 .clone 方法
*/
fn clone_deep_copy(){
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

/*
展示函数调用过程中所有权转移的例子
*/
fn function_call_ownship_change(){
    let s = String::from("hello");
    takes_ownship(s);       // 做一个测试，此时所有权转移到 takes_ownship 的 some_string, 调用完之后被 drop 掉，按道理 s 不能再次被使用

    // println!("[Test - Move] ownership of s: String: {}", s);
    // 报错: value borrowed here after move

    let x = 5;
    make_copy(x);           // 做一个测试，此时 x 发生的只是浅拷贝，按道理传入的参数 copy 之后，还可以调用原来的 x

    println!("[Test - Copy] ownership of x: i32: {}", x);
}

fn takes_ownship(some_string: String){
    println!("takes_ownership: some_string:{}", some_string);
}

fn make_copy(x: i32){
    println!("make_copy: {}", x);
}

/*
函数作为返回值，对所有权的传递
*/
fn function_return_ownership_change(){
    let _s1 = give_a_string();

    let s2 = String::from("HAHA");
    let s3 = take_a_string(s2);

    // println!("[Test - s2] : {}", s2);  
    // Error: --move occurs because `s2` has type `String`, which does not implement the `Copy` trait
    // Error: value borrowed here after move

    println!("[Test - s3]: {}", s3);
}

fn give_a_string() -> String{
    let some_string = String::from("hallo");
    some_string
}

fn take_a_string(a_string: String) -> String {
    a_string 
}


/*
Rust 中的部分引用
通过 ref 的作用，获得结构中某个成员的引用
尽管结构体的整体不能再被使用，但是单个成员依然可以被索引
*/
fn partial_reference(){
    struct Person{
        name: String,
        age: Box<u8>,
    }

    let person = Person{
        name: String::from("Alice"),
        age: Box::new(20),
    };

    let Person {name, ref age} = person;
    /*
    这里应该是从 person 这个实例中，获得 name(转移 Alice 所有权)，以及 age 的引用
     */
    println!("Get person's name: {}", name);
    println!("Get person's age: {}", age);

    println!("Get from person.age: {}", person.age);
    // println!("Get total struct: {:?}", person);
}


// Exercise part
// *********************** 使用尽可能多的方法来通过编译 ***********************************
// let y = x;
fn exerise_1_1(){
    let x = String::from("hello, world");
    let y = x.clone();          // 直接深拷贝
    println!("exerise_1_1 {}  ,{}", x, y);
}

fn exerise_1_2(){
    let x = "hello, world";     // 都是字符串字面量的引用，实际都不具有 "hello, world 的所有权", 字符串字母量不可变，可以创建多个引用指向安全地指向它
    let y = x;
    println!("exerise_1_2 {}  ,{}", x, y);
}

fn exerise_1_3(){
    let x = &String::from("hello, world");      // 是对一个 String 的引用，只是复制了引用
    let y = x;
    println!("exerise_1_3 {}  ,{}", x, y);   // 依然是引用的操作
}


fn exerise_1_4(){
    let x = String::from("hello, world");
    let y = x.as_str();                 // as_str() 方法返回一个指向 String 内部数据的不可变引用 &str, 不会改变 x 的所有权， y 是对 x 中数据的一个引用
    println!("exerise_1_4 {}  ,{}", x, y);
}

/*
当所有权转移时，可变性也可以随之改变
*/
fn exercise_5(){
    let x = Box::new(5);
    
    // 创建另一个可变的 Box, 同样在堆上分配，初始值为整数 3
    let mut y = Box::new(3);
    *y = 4;
    // 解引用 y 并将其值设置为 4, Box<T> 是一个智能指针，用于在堆上分配内存（这里是整数), mut 关键字使得 y 可变。

    assert_eq!(*x, 5);
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

    clone_deep_copy();

    // 测试 move 和 copy 在函数调用 传参中产生的所有权转移
    function_call_ownship_change();

    // 测试 move 在函数返回值 中产生的所有权转移
    function_return_ownership_change();

    partial_reference();

    // Exercise part
    // Exec. 1
    exerise_1_1();
    exerise_1_2();
    exerise_1_3();
    exerise_1_4();

    exercise_5();
}