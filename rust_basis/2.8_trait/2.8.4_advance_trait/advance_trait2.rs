/*
特征部分关联类型的对比，分别实现 关联类型的 container 和特征类型的 container
*/
use std::fmt;
pub trait AssociatedContainer{
    type A : fmt::Display + PartialOrd + PartialEq;
    type B : fmt::Display + PartialOrd + PartialEq;

    fn contains(&self, x: &Self::A, y: &Self::B) -> bool;
}

struct ContainerEx1{
    source: i32,
    destation: i32,
}

impl AssociatedContainer for ContainerEx1{
    type A = i32;
    type B = i32;

    fn contains(&self, x: &Self::A, y: &Self::B) -> bool{
        if x == y {
            println!("source: {} = dest:{}",x, y);
            true
        }else{
            println!("[Error] source:{} != dest:{}", x, y);
            false
        }
    }
}

// 只用关联类型编写，相对实现起来并不需要在 AssociatedContainer 之上再加 A, B 的泛型类型约束
fn associated_container_test(){
    let ctn = ContainerEx1{source: 13, destation: 20};
    ctn.contains(&ctn.source, &ctn.destation);
}

// vs 下面的部分都用泛型实现，内部没有关联类型
pub trait GenericContainer<A: fmt::Display + PartialEq + PartialOrd, B: fmt::Display + PartialEq + PartialOrd>{
    fn contains(&self, x: &A, y: &B) -> bool;
}

struct ContainerEx2<A, B>
    where A: fmt::Display + PartialEq + PartialOrd,
          B: fmt::Display + PartialEq + PartialOrd,
{
    source: A,
    destation: B,
}

impl GenericContainer<f64, f64> for ContainerEx2<f64, f64>
{
    fn contains(&self, x: &f64, y: &f64) -> bool{
        if x <= y {
            println!("from source {} to dest {}", x, y);
            true
        }else{
            println!("[Another] dest {} to source{}", x, y);
            false
        }
    }
}

fn generic_container_test(){
    let cnt = ContainerEx2{source: 3.1415926, destation: 2.4879234};
    cnt.contains(&cnt.source, &cnt.destation);
}

// 实际上主要的 difference 在于外部定义时
// 1. 函数参数是 AssociatedContainer 类型
fn difference1<C>(_container: &C)
    where C: AssociatedContainer,
{
    println!("difference1 of container!");
}

// 2. 函数参数是 GenericContainer 类型
fn difference2<A,B,C>(_container: &C)
    where A: fmt::Display + PartialEq + PartialOrd,
          B: fmt::Display + PartialEq + PartialOrd, 
          C: GenericContainer<A,B>,
{
    println!("difference2 of container!");
}

fn main(){
    associated_container_test();    // 关联类型实现 Container
    generic_container_test();       // 普通泛型实现 Container

    let cnt1 = ContainerEx1{source: 12, destation: 32};
    difference1(&cnt1);
    let cnt2 = ContainerEx2{source: 187.34853, destation: 173.4239};
    difference2(&cnt2);
}
/* 这里在设计上有点欠考虑，导致 即使有了关联类型或者 A,B, 但实际上外部的 difference1/2 并不能从实现了某个特征的类型中偷到一些信息 
? 后续做一些修改? 但目前认为 type A 和 type B 实际上仅仅起到类似 java 中的封装功能, 似乎并不能起到存储一些有效值的目的
试图修改 difference1/2, 发现调用 _container.contains(&_container::A, &_container::B)
并不能实现期望功能，找不到对应的模块值 [一个修改是增加 std::default::Default 特性]  --> 但实际这个值和 container 没什么关系
_container.contains(&_container::A, &_container::B); // error
*/