/*
将 Vec<T> 原本的 ptr 和 cap 字段抽出来，组成一个更底层的内存分配
Vec<T> 的某些 push, pop 和 next, next_back 的 Into_Iter 操作中，实际并不需要关心底层的内存
以及 Vec<T> 中的缓冲区在 IntoIter 中也存在被释放两次的问题
改写 Vec<T> 的字段，封装更为底层的 RawVec<T> 字段
*/
//[part 1]. 声明 RawVec<T> struct，new 和 grow() 操作放在 RawVec<T> 中完成, RawVec<T> 自己有一个释放内存的 Drop 方法 
use std::ptr::NonNull;

pub struct RawVec<T>{
    ptr: NonNull<T>,
    cap: usize,
}

unsafe impl<T: Send> Send for RawVec<T>{}
unsafe impl<T: Sync> Sync for RawVec<T>{}

use std::mem;

use std::alloc::{self, Layout};

impl<T> RawVec<T>{
    fn new() -> Self{
        assert!(mem::size_of::<T>() != 0, "TODO: implement ZTS!");
        RawVec{
            ptr: NonNull::dangling(),
            cap: 0,
        }
    }

    fn grow(&mut self){
        let new_cap = if self.cap == 0 {1} else{ 2 * self.cap};
        let new_layout = Layout::array::<T>(new_cap).unwrap();

        assert!(new_cap.size() <= isize::MAX as usize, "Allocation too large!");

        let new_ptr = if self.cap == 0 {
            unsafe{ alloc::alloc(new_layout)}
        }else{
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            unsafe{
                alloc::realloc(old_ptr, old_layout, new_layout.size())
            }
        };

        self.ptr = match NonNull::new(new_ptr as *mut T){
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };

        self.cap = new_cap;
    }
}

impl<T> Drop for RawVec<T>{
    fn drop(&mut self){
        if self.cap != 0 {
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe{
                alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}

// [part 2]. 声明带有 RawVec<T> 字段的 Vec<T>, 这里不需要再考虑内存分配的 grow() 功能，drop 方法也相对简单
pub struct Vec<T>{
    buf: RawVec<T>,
    len: usize,
}

impl<T> Vec<T>{
    fn new() -> Self{
        Vec{
            buf: RawVec::new(),
            len: 0,
        }
    }

    pub fn ptr(&self) -> *mut T{
        self.buf.ptr.as_ptr()
    }

    pub fn cap(&self) -> usize{
        self.buf.cap
    }
}

impl<T> Drop for Vec<T>{
    fn drop(&mut self){
            while let Some(_) = self.pop(){}
            // 后续的内存释放交给 RawVec<T>
    }
}

// [part 3]. 实现带有 RawVec<T> 字段的 IntoIter<T>
pub struct IntoIter<T>{
    _buf: RawVec<T>,        // 这里 _buf 并不需要直接被使用
    start: *const T,
    end: *const T,
}

impl<T> IntoIterator for Vec<T>{
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T>{
        let buf = unsafe{ ptr::read(&self.buf) };
        let len = self.len;
        let cap = buf.cap;
        
        mem::forget(self);

        IntoIter{
            _buf: buf,
            start: buf.ptr.as_ptr(),
            end: if cap == 0 {
                buf.ptr.as_ptr()
            }else{
                unsafe{buf.ptr.as_ptr().add(len)}
            },
        }
    }
}

impl<T> Drop for IntoIter<T>{
    fn drop(&mut self){
        // 反复调用 IntoIter 的 next 方法
        for _ in &mut *self {}
    }
}