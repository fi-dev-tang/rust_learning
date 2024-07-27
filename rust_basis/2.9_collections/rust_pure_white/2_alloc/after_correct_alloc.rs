/* 在纠错一遍之后，进行 new() 和 grow() 功能的重写 */
use std::ptr::NonNull;

pub struct Vec<T>{
    ptr: NonNull<T>,
    len: usize,
    cap: usize,
}

unsafe impl<T: Send> Send for Vec<T>{}
unsafe impl<T: Sync> Sync for Vec<T>{}

use std::mem;

impl<T> Vec<T>{
    fn new() -> Self{
        assert!(mem::size_of::<T>() != 0, "We do not implement ZTS" );
        Vec{
            ptr: NonNull::dangling(),
            len: 0,
            cap: 0,
        }
    }
}

use std::alloc::{self, Layout};

impl<T> Vec<T>{
    fn grow(&mut self){
        let (new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::array::<T>(1).unwrap())
        }else{
            let new_cap = self.cap * 2;
            let new_layout = Layout::array::<T>(new_cap).unwrap();
            (new_cap, new_layout)
        };

        assert!(new_layout.size() <= isize::MAX as usize, "alloc too large");

        let new_ptr = if self.cap == 0 {
            unsafe { alloc::alloc(new_layout)}
        }else{
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe {alloc::realloc(old_ptr, old_layout, new_layout.size())}
        };

        self.ptr = match NonNull::new(new_ptr as *mut T){
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };

        self.cap = new_cap;
    }
}