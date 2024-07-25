pub struct Vec<T>{
    ptr: *mut T,
    cap: usize,
    len: usize,
}

/*
NonNull 是对原始指针的一种包装: 在 T 上是协变的，并且指针永不为空
*/
use std::ptr::NonNull;
pub struct Vec<T>{
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
}

unsafe impl<T: Send> Send for Vec<T>{}
unsafe impl<T: Sync> Sync for Vec<T>{}