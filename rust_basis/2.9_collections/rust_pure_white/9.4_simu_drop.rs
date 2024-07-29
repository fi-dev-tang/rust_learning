/*
实现动态数组 Vec<T> 的 drop 方法
drop 方法的实现分为两步:
1. 第一步，逐个调用 pop() 移动出动态数组中的元素，在这个过程中，实现了 Drop 特性的 T 会对这些元素进行析构，
但如果是简单的 i32 类型，并不需要析构，则 pop() 方法调用不必要，LLVM会将这部分代码优化掉。
2. 第二步，对动态数组 Vec<T> 原先申请的内存进行释放，调用 unsafe{ alloc::dealloc()}
*/
use std::alloc::{self, Layout};

impl<T> Drop for Vec<T>{
    fn drop(&mut self){
        if self.cap != 0 {
            while let Some(_) = self.pop() {}
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe{
                alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}