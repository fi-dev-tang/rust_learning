/*
泛型
*/
fn add_i8(a: i8, b: i8) -> i8{
    a + b
}

fn add_i32(a: i32, b: i32) -> i32 {
    a + b
}

fn add_f64(a: f64, b: f64) -> f64 {
    a + b
}

fn not_recommand(){
    println!("add_i8 : {}", add_i8(1i8, 2i8));
    println!("add_i32 : {}", add_i32(10i32, 20i32));
    println!("add_f64: {}", add_f64(1.0, 2.0));
}

// 自己写一个泛型加法的例子
fn add_trait<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn implement_trait_add(){
    println!("add i8 {}",  add_trait(1i8, 2i8));
    println!("add i32 {}", add_trait(10i32, 20i32));
    println!("add f64 {}", add_trait(1.23, 2.23));
}


// 自己写一个泛型比较数组中最大元素的例子
fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &iter in list.iter() {
        /*
        这里特别介绍一下 &iter:
        由于 list.iter() 返回一个迭代器对象，迭代器对象返回的其实是其中每个元素的 引用
        所以使用 &iter , 之后的 iter 相当于解引用，直接使用值
        等价于 for iter_ref in list.iter(){
            if *iter_ref > largest {
                largest = iter;
            }
        }
        */
        if iter > largest{
            largest = iter;
        }
    }
    largest
}

fn implement_trait_largest(){
    let number_list = vec![1, 4, 9, 2, 3, 18, 6];
    println!("the largest in number_list: {}", largest(&number_list));

    let character_list = vec!['a', 'c', 'q', 'h'];
    println!("the largest in character_list: {}", largest(&character_list));
}

fn main(){
    not_recommand();                // 没有泛型的不推荐写法

    implement_trait_add();          // 用泛型实现加法
    implement_trait_largest();      // 用泛型实现数组比较
}

/*
解释问题
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> T {
    let mut largest = list[0];      // 1. 泛型类型 `T` 必须实现 `Copy` trait 才能按值移动
                                    // 2. 使用了 for &iter in list.iter() 这种尝试解构，适用于基本类型(char, i32) 这里会涉及所有权的问题。
    for &iter in list.iter() {
        if iter > largest {
            largest = iter;
        }
    }
    largest
}

1. let mut largest = list[0];
这涉及将 list[0] 的值按值复制到 largest 中，对于大部分类型，按值赋值需要实现 Copy trait，否则编译器会报错，例如试图按值返回不安全的引用。
2. 解引用问题
for &iter in list.iter() 语法要求 `T` 类型实现 `Copy` trait, 因为 `iter` 是通过引用解构的。
如果 `T` 类型没有实现 Copy trait, 编译器会抱怨所有权和借用问题。


*/