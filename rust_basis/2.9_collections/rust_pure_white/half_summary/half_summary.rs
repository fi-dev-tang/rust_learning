/*
对死灵书实现 Vec<T> 前五节的总结
1. 类型定义和布局
2. 内存分配 (grow) --> 篇幅最多
3. push 和 pop
4. drop
5. Deref 和 DerefMut
*/
//------------------------- part 1 Vec<T> 结构体的类型定义和布局 ------------------------------
use std::ptr::NonNull;

pub struct Vec<T>{
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
}

unsafe impl<T: Send> Send for Vec<T>{}
unsafe impl<T: Sync> Sync for Vec<T>{}

//------------------------- part2 内存分配 grow()----------------------------------------------
// grow 需要分成两个部分，self.cap == 0 的 alloc 和 self.cap != 0 的 realloc
// 涉及到对 self.cap 的修改和 self.ptr 的修改，需要严格保证 self.ptr 不为空，同时 self.cap 严格小于等于 isize::MAX

use std::alloc::{self, Layout};
use std::mem;

impl<T> Vec<T>{
    fn new() -> Self{
        assert!(mem::size_of::<T>() != 0, "We do not consider ZTS");
        Vec{
            ptr: NonNull::dangling(),
            cap: 0,
            len: 0,
        }
    }
}

impl<T> Vec<T>{
    fn grow(&mut self){
        let (new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::array::<T>(1).unwrap())
        }else{
            let new_cap = 2 * self.cap;
            let new_layout = Layout::array::<T>(new_cap).unwrap();
            (new_cap, new_layout)
        };

        assert!(new_layout.size() <= isize::MAX as usize, "Allocation too large!");

        let new_ptr = if self.cap == 0 {
                unsafe{ alloc::alloc(new_layout)}
            }else{
                let old_ptr = self.ptr.as_ptr() as *mut u8;
                let old_layout = Layout::array::<T>(self.cap).unwrap();
                unsafe{
                    alloc::realloc(old_ptr, old_layout, new_layout.size())
                }
        };

        let self.ptr = match NonNull::new(new_ptr as *mut T){
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.cap = new_cap;
    }
}

//----------------------------- part 3 pop() and push() ----------------------------------
// 不能直接解引用，会涉及错误解析某些非 T 类型的垃圾值  ptr::read 和 ptr::write

impl<T> Vec<T>{
    fn push(&mut self, elem: T){
        if self.cap == self.len { self.grow();}
        unsafe{
            ptr::write(self.ptr.as_ptr().add(self.len), elem);
        }
        self.len += 1;
    }
}


impl<T> Vec<T>{
    fn pop(&mut self) -> Option<T>{
        if self.cap == 0 {
            None
        }else{
            self.len -= 1;
            unsafe{
                Some(ptr::read(self.ptr.as_ptr().add(self.len)))
            }
        }
    }
}

//-------------------------------- part 4 drop() ---------------------------------------------
// 1. pop() 2. dealloc
use std::alloc;
impl<T> Drop for Vec<T>{
    fn drop(&mut self){
        if self.cap != 0 {
            while let Some(_) = self.pop() {}
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe{
                alloc::dealloc(self.ptr.as_ptr() as *mut u8,layout);
            }
        }
    }
}

//------------------------------- part5 Deref && Dere_mut -----------------------------------------
// 返回 Vec<T> 的不可变引用和可变引用数组切片
use std::ops::Deref;
use std::ops::DerefMut;

impl<T> Deref for Vec<T>{
    type target = [T];
    fn deref(&self) -> &target{
        std::slice::from_raw_parts(self.ptr.as_ptr(), self.len)
    }
}

impl<T> DerefMut for Vec<T>{
    fn deref_mut(&mut self) -> &mut [T]{
        std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len)
    }
}