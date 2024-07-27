/* 尝试自己写一遍(死灵书)中关于 std::vec::Vec<T> 的实现(包括设计布局和内存分配)
--> 按照自己记录的思路
(这一版是错误百出的实现版，目的是和正确内容做一个对照)
*/
// 第一部分: 实现 Vec<T> 的设计布局
use std::ptr::NonNull;      // 特殊设计引进，保证指向数组的指针始终不为空(可以存储垃圾值)

struct Vec<T>{
    ptr: NonNull<T>,       // 实际本质类型是 *mut T
    cap: usize,
    len: usize,
}

unsafe impl<T: Send> Send for Vec<T>{}
unsafe impl<T: Sync> Sync for Vec<T>{}

// 第二部分：实现内存分配
// 内存分配不需要管结构体中 len 的部分，只需要记录 ptr 和 cap 的修改
use std::alloc::{Self, Layout};
use std::mem;

impl<T> Vec<T>{
    // 1. 创建一个空的 Vec<T> 作为返回值   (关联函数 -> Self)
    fn new() -> Self{
        // 不处理类型大小为零的 T 
        assert!(mem::size_of::<T>() != 0, "We do not handle Zero-Size-Types!");

        Vec<T>{
            ptr : NonNull::dangling(),   // 悬垂指针
            cap : 0,
            len : 0,
        }
    }
}

// 第二部分，判断内存申请情况 1. self.cap <= isize::MAX, 2. self.ptr 不能为空
impl<T> Vec<T>{
    fn grow(&mut self) -> {
        let (new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::arrays::size_of::<T>(1)),
        }else{
            let new_cap = 2 * self.cap;
            // 逻辑是这里的 Layout 不会报错，因为始终有 self.cap <= isize::MAX, 所以 最多达到 2 * isize::MAX, 小于 usize::MAX,
            let new_layout = Layout::arrays::size_of::<T>(new_cap).unwrap();
            (new_cap, new_layout),
        };

        assert!( new_cap < isize::MAX as usize , "new_capacity out of memory limits!");     // 这里边界条件会出错，但也从而保证了 self.cap 始终小于 isize::MAX

        let old_ptr = self.ptr;
        let old_cap = self.cap;
        let self.ptr = if new_cap == 1 {
            alloc::allocate(1, new_layout),
        }else{
            alloc::realloc(old_ptr, Layout::arrays::size_of::<T>(old_cap), new_cap, new_layout),
        }
        
        // 最后判断一下 self.ptr 是否为空
        let self.ptr = 
            match self.ptr.as_ptr() as *mut u8 {
                Some(p) => p,
                None => NonNull::handle_ptr_error(),        // 这里会报错退出
            };
        self.cap = new_cap;
    }
}