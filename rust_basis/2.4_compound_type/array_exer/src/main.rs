/* 
实现数组相关的知识点。
静态数组: array, 动态数组 Vector
静态数组长度固定，存储在栈上，性能优越。 动态数组长度可变，存储在堆上
*/
fn create_array(){
    let a = [1,2,3,4,5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    // 语法: [类型; 长度]
    let b: [i32; 5] = [1,2,3,4,5];
    let c = [3;5];
    // 初始化一个某个值重复出现了 N 次的数组 [类型; 长度] 对应 [3; 5]

    println!("a = {:?}", a);
    println!("months = {:?}", months);
    println!("b = {:?}", b);
    println!("c = {:?}", c);
}

/*
Rust: 运行时检查访问索引是否超过或等于数组长度
应用：请用户输入一个下标，作为数组索引
*/
use std::io;
fn array_index(){
    let a: [i32; 5] = [1,2,3,4,5];
    
    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("Not valid string");

    let index : usize = index.trim().parse().expect("Not valid number");

    let element = a[index];

    println!("The element at index {} is {}", index, element);
}

// 数组元素是非基础类型时，如何访问
/*
let a = [String::from("Hello rust!"); 8];
println!("{:#?}", a);
出现报错: the trait `Copy` is not implemented for `String`

所有权知识可以解释: 前面几个例子都是 Rust 的基本类型，基本类型在 Rust 中赋值是以 Copy 的形式
let array = [3; 5] 底层就是不断 Copy 出来的，但复杂类型都没有深拷贝，只能一个个创建
*/
fn not_basic_type(){
    //let a = [String::from("Hello rust!"),String::from("Hello rust!"),String::from("Hello rust!"),String::from("Hello rust!"),
    //        String::from("Hello rust!"),String::from("Hello rust!"),String::from("Hello rust!"),String::from("Hello rust!")];
    //println!("{:#?}", a);
    // 这种写法很冗余, 可以使用 std::array::from_fn 构造 String 数组
    let array: [String; 8] = std::array::from_fn( |_i| String::from("Hello rust!"));
    println!("{:#?}", array);
}

// 使用数组切片
fn array_slice(){
    let a: [i32; 5] = [1,2,3,4,5];
    let slice: &[i32] = &a[1..3];
    assert_eq!(slice, &[2,3]);
    println!("slice {:#?}", slice);
}

// 数组使用总结
fn array_summary(){
    // 第一部分: 自动类型推导
    let one = [1,2,3];
    let two: [u8; 3] = [1,2,3];
    let blank1 = [0;3];
    let blank2: [u8; 3] = [0; 3];

    let array: [[u8;3]; 4] = [one, two, blank1, blank2];
    
    for a in &array{
        print!("{:?}:", a);

        for n in a.iter() {
            print!("\t {} + 10 = {}",n, n + 10);
        }

        let mut sum = 0;
        for n in a.iter(){
            sum += n;
        }

        println!("\t({:?} = {})", a, sum);
    }
}

fn main(){
    create_array();     // 数组创建
    array_index();      // 获取用户下标进行 array 索引

    not_basic_type();   // 非基础类型
    array_slice();      // 数组切片

    array_summary();    // 数组使用总结
}