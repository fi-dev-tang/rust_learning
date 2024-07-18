use std::convert::TryInto;
use std::ops::Add;

// 修复 largest 的错误
// 第一种解决方案: 添加 Copy trait
fn largest1<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// 第二种解决方案，并不强制能够在堆上进行复制分配，使用 Clone() 特性对每个值进行复制，使得 largest 拥有所有权
/*
在写法上的注意点，第二种实现方式是 Clone trait, 而不是 Copy trait
原因:
for &item in list.iter(), 会将每一个 `item` 进行解引用，即将 `&T` 转换为 `T`, 这要求 `item` 类型必须实现 `Copy` trait。
使用解引用方式必须具备 Copy 的特性。
*/
fn largest2<T: std::cmp::PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();

    for item in list.iter(){
        if *item > largest{
            largest = item.clone()
        }
    }
    largest
}

/*
第三种解决方案，不需要 Clone 或 Copy trait, 无需堆分配
直接返回引用的传递
Rust 一般的引用比较是比较指向的内存地址
如果实现了 std::cmp::PartialOrd 特征，实际上委托了 T 类型的比较，来比较实际元素的大小
*/
fn largest3<T: std::cmp::PartialOrd>(list: &[T]) -> &T{
    let mut largest = &list[0];
    for item in list.iter(){
        if item > largest{
            largest = item;
        }
    }
    largest
}

fn using_largest(){
    let mut number_list = vec![1,2,3,4,5,6];
    let mut character_list = vec!['a', 'c', 'd', 'f', 'm'];

    println!("[Copy trait] number_list: {}", largest1(&number_list));
    println!("[Copy trait] character_list: {}", largest1(&character_list));

    println!("[Clone trait] number_list: {}", largest2(&number_list));
    println!("[Clone trait] character_list: {}", largest2(&character_list));

    println!("[No trait] number_list: {}", *largest3(&mut number_list));
    println!("[No trait] character_list: {}", *largest3(&mut character_list));
}

// 使用 try_into() 进行数据的转换，和 as 相比有完全的控制权和错误处理
// 必须将 std::convert::TryInto 引入当前的作用域中，因为需要使用该特征的方法时，必须将该特征引入当前作用域中
fn using_try_into_in_current_namespace(){
    let a : i32 = 10;
    let b : u8 = 100;

    let b_ = b.try_into().unwrap();

    if a < b_ {
        println!("ten is less than a hundred!");
    }
}

// 综合例子 1. 使用加法运算符，而不是全局的 add 来完成自定义加法
#[derive(Debug)]
struct Point<T: Add<T, Output = T>>{
    x: T,
    y: T,
}

impl<T: Add<T, Output = T>> Add for Point<T>{
    type Output = Point<T>;

    fn add(self, p: Point<T>) -> Point<T>{
        Point{
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}

fn add<T: Add<T, Output = T>>(a: T, b: T) -> T {
    a + b
}

fn adding_example(){
    let point1 = Point{x : 1.1f32, y: 1.1f32};
    let point2 = Point{x : 1.1f32, y: 1.1f32};

    let point3 = Point{x: 2u16, y: 3u16};
    let point4 = Point{x: 2u16, y: 3u16};

    println!("{:?}", add(point1, point2));
    println!("{:?}", add(point3, point4));
}

// 综合例子 2. 实现文件打印
use std::fmt;
use std::fmt::Display;
#[derive(Debug)]
enum FileState{
    Open,
    Closed,
}

#[derive(Debug)]
struct File{
    name: String,
    data: Vec<u8>,
    fstate: FileState,
}

impl Display for FileState{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match *self{
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

impl Display for File{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "<{}, {}>", self.name, self.fstate)
    }
}

impl File{
    fn new(filename: &str) -> File{
        File{
            name: String::from(filename),
            data: Vec::new(),
            fstate: FileState::Closed,
        }
    }
}

fn displaying_example(){
    let f6 = File::new("f6.txt");
    println!("{:?}", f6);
    println!("{}", f6);
}

fn main(){
    using_largest();                        // 三种特征形式实现 largest 1. Copy 特征，for &item in list.iter(), 解引用需要 Copy 特征
                                            // 2. Clone 特征，无法直接解引用(&T -> T 过程有 copy, 可以使用 for item in list.iter() , *iter > largest, largest = iter.clone())
                                            // 3. 无需 Copy 或 Clone 特征，直接进行Rust引用的比较(引用比较的根据是 PartialOrd trait, 同样可以比较数值大小)
    using_try_into_in_current_namespace();  // 使用某个特征的方法时，必须令特征在当前作用域中

    adding_example();                       // 综合例子，自定义加法
    displaying_example();                   // 综合例子，自定义打印
}