/*
完善对于 const 泛型表达式的理解
*/
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub enum Assert<const CHECK: bool>{
    //
}

pub trait IsTrue{
    //
}

impl IsTrue for Assert<true>{
    //
}

fn something<T>(val: T)
where
    Assert<{core::mem::size_of::<T>() < 768}>: IsTrue,
{
    // 其中 Assert<> 内部中的 {} 是 const 泛型表达式
}

fn main(){
    something([0u8; 0]);
    something([0u8; 512]);
    something([0u8; 1024]);
}