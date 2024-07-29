/*
为 Vec<T> 实现 Deref 和 DerefMut 特性
功能特点: 从目标的 Vec<T> 中返回某个元素的索引或者切片类型 &[T]
例如从 Vec<i32> 中返回某个元素的索引 &i32
*/
use std::ops::Deref;
use std::ops::DerefMut;

impl<T> Deref for Vec<T>{
    type target = [T];
    fn deref(&self) -> &target {
        unsafe{
            std::slice::from_raw_parts(self.ptr.as_ptr(), self.len);
        }
    } 
}

impl<T> DerefMut for Vec<T>{
    fn deref_mut(&mut self) -> &mut [T]{
        unsafe{
            std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len);
        }
    }
}