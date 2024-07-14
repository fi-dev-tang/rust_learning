#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
// 回顾 结构体和方法上的泛型实现
struct SingleModulePoint<T>{
    x: T,
    y: T,
}

impl<T> SingleModulePoint<T>{
    fn x(&self) -> &T{
        &self.x
    }

    fn y(&self) -> &T{
        &self.y
    }
}

fn implement_generics_on_method(){
    let number_point = SingleModulePoint{x: 10, y: 10};
    println!("number_point x = {}, y = {}", number_point.x(), number_point.y());

    let character_point = SingleModulePoint{x: 'c', y: 'h'};
    println!("character_point x = {}, y = {}", character_point.x(), character_point.y());
}

// 回顾结构体和方法中添加多种泛型参数
struct MultiModulePoint<T : Copy,U : Copy>{
    x: T,
    y: U,
}

impl<T : Copy,U : Copy> MultiModulePoint<T ,U>{
    fn x(&self) -> &T{
        &self.x
    }

    fn y(&self) -> &U{
        &self.y
    }

    // 方法上也可以使用泛型参数，这里既有结构体上的泛型参数，也有方法上的泛型参数
    fn mixup<V : Copy,W : Copy>(self: &MultiModulePoint<T,U>, other: &MultiModulePoint<V,W>) -> MultiModulePoint<T,W>{
        MultiModulePoint{
            x: self.x,
            y: other.y,
        }
    }
}

fn implement_multi_generics_on_method(){
    let p1 = MultiModulePoint{x: "Hello Rust", y: 'c'};
    let p2 = MultiModulePoint{x: 12.4, y: 13};

    let p3 = (&p1).mixup(&p2);      // 和昨天写法的不同，为了在之后仍然能够打印 p1 和 p2, 换言之，不会把所有权传递进函数中，让后续所有权消失
                                    // 因此都需要传递引用, 复制当前引用中的值给新的变量时，需要检验是否能够满足 Copy trait(重新创建新值浅拷贝)，
                                    // 为 T, U 泛型添加 Copy 特性的限制，V,W 同样
    println!("p1's x = {}, p2's y = {}", p1.x(), p2.y());
    println!("p3's x = {}, p3's y = {}", p3.x(), p3.y());
}

// 可以针对特定的泛型类型实现具体的方法
impl SingleModulePoint<f32>{
    fn distance_from_origin(&self) -> f32{
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn implement_specific_generic_on_method(){
    let point_f32 = SingleModulePoint{x: 3.0f32, y: 4.0f32};
    println!("[distance_from_origin] point_f32's x = {}, y = {}, distance = {}", point_f32.x(), point_f32.y(), point_f32.distance_from_origin());
}

// const generic 部分
/*
display 函数，如果写死，参数传递为: arr: &[i32; 3]
则对于所有不同长度的 arr 打印，只能打印值为 3 的长度的数组 -> const generic: 希望实现值不同的泛型
一种简单的解决方案: 传递不定长度的数组，参数为不可变引用的数组切片 arr: &[i32]
进一步推广，改成泛型
*/
// 实现方式一: 在可以使用不可变引用，以及数组切片的前提下
/*
fn display_array<T: std::fmt::Debug>(arr: &[T]){
    println!("{:?}", arr);
}
调用通过 display_array(&arr);
*/
// 实现方式二: 引入 const 泛型，对数组的长度值进行泛型限制，传入的参数就是数组本身，不含引用
fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T ; N]){
    println!("{:?}", arr);
}

fn display_multi_num_array(){
    let array1: [i32; 3] = [1, 2, 3];
    display_array(array1);

    let array2 : [i32; 2] = [1, 2];
    display_array(array2);

    let array3 : [char; 5] = ['a', 'b', 'c', 'd', 'e'];
    display_array(array3);

    let array4 : [String; 6] = ["Hello world".to_string(), "Hello Rust".to_string(), "Rust-by-Example".to_string(),
                                "Rust Course".to_string(), "Welcome to rust".to_string(), "rust handling well".to_string()];
    display_array(array4);
}

// 使用 const 泛型表达式的部分
pub enum Assert<const CHECK: bool>{
    // 枚举类型 Assert<> 接受 const 表达式判断
}

pub trait IsTrue{
    // 定义 IsTrue 的特征
}

impl IsTrue for Assert<true>{
    // 为特定类型的const 泛型 Assert<True> 实现 IsTrue 特征
}

fn something<T>(_val: T)
where
    Assert< { core::mem::size_of::<T>()  < 768 }    >: IsTrue,
{
    // 希望 Assert<> 内部的 const 表达式的值为 true
    /*
    core::mem::size_of::<T>()
    的具体使用：希望程序在内存很小的平台上运行，因此通过 core::mem::size_of::<T>() < 768, 
    限制能够使用的内存大小，如果 const 泛型表达式 (generic const expression) 的值为 true, 那么为其实现了 IsTrue 的特征，程序正常执行
    */
}

fn implement_generic_const_expression_for_mem(){
    something([0u8; 0]);    // ok
    something([0u8; 512]);  // ok
    // something([0u8; 1024]); // not
    /*
    编译器显示的报错结果: something([0u8; 1024]); expected `false`, found `true`
    where Assert< {core::mem::size_of::<T>() < 768} >: IsTrue,
    */ 
}

fn main(){
    // 回顾部分
    implement_generics_on_method();          // 在方法上实现泛型
    implement_multi_generics_on_method();    // 在方法上实现多种泛型

    // 新增部分
    implement_specific_generic_on_method(); // 针对特定泛型类型，实现具体的方法，除此之外的泛型无此用法
    // const generic
    display_multi_num_array();              // 引入 const 泛型 [T; N], 其中 T 代表泛型的类型，N 代表泛型的数目
    // const exprs
    implement_generic_const_expression_for_mem();       // 为了控制平台的内存消耗，使用 const 泛型表达式
}