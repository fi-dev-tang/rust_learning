// 这个实现有语法错误！
// 布局上的设计
// 原始版本 1.
struct Vec<T>{
    ptr: *mut T,    // 动态数组的指针指向
    cap: usize,     // 动态数组申请的容量
    len: usize,     // 当前长度
}

/*
ptr: *mut T 的问题，
原本的 std::vec::Vec<T> 是用 Unique<T> 不稳定版本改进 *mut T, Unique<T> 的好处
1. 当 T 实现 Send 或 Sync 特征时，Unique T 自动实现
2. 可以获得 T 类型的数值
3. 非空指针的约束，确保所有的 Option<Vec<T>> 不会出现空指针的情况

基于这三点，重新进行稳定实现, 使用 NonNull
*/
// 重新实现版 2.
use std::NonNull;

struct Vec<T>{
    ptr: NonNull<T>,
    cap: usize, 
    len: usize,
}

pub impl Send for Vec<T>{}
pub impl Sync for Vec<T>{}