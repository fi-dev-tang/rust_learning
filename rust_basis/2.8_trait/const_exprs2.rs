/* 使用 const 泛型表达式: 小平台限制内存大小的实践  */
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub enum Assert<const CHECK: bool>{

}

pub trait IsTrue{

}

impl IsTrue for Assert<true>{

}

fn something<T>(_val: T)
where
    Assert< {core::mem::size_of::<T>() < 768} >: IsTrue,
{

}

fn main(){
    something([0u8; 0]);
    something([0u8; 512]);
    something([0u8; 1024]);
}