// 9.1 合并 9.2 节的模板
// 模板文件不是自己写的，直接从(死灵书)的写法中得到
/*
分配内存的总思路概括:
1. grow 的逻辑
if cap == 0:
    allocate()
    cap = 1
else:
    reallocate()
    cap *= 2
2. 依据 LLVM 的特性对内存分配添加控制和约束
例如分配内存的大小不超过 isize::MAX (可能与数组从反方向取偏移量有关,所以为 isize 的有符号偏移类型)
2. 始终保持 Vec<T> 的 ptr 指针满足 NonNull 的限定(Optioni<Vec<T>> 不为空)
3. 初始值还没有分配指针，同时希望指针不为空（也就是限制这个指针确实有值，但是是一个没有任何意义，
不指向任何有意义值的垃圾数值)
*/
use std::ptr::NonNull;

pub struct Vec<T>{
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
}

unsafe impl<T: Send> Send for Vec<T>{}
unsafe impl<T: Sync> Sync for Vec<T>{}

use std::mem;

impl<T> Vec<T>{
    pub fn new() -> Self{
        assert!(mem::size_of::<T>() != 0, "We're not ready to handle zero-size-type s");
        Vec{
            ptr: NonNull::dangling(),   // 悬垂指针
            len: 0,
            cap: 0,
        }
    }
}

use std::alloc::{self, Layout};

impl<T> Vec<T>{
    fn grow(&mut self){
        // 1. 第一步，设置新的 new_cap 和 new_layout
        // 这一步设置新的容量和申请空间大小 --> new_layout 针对 layout 格式展开更具体的空间大小和对齐要求
        let (new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::array::<T>(1).unwrap())
        }else{
            // 因为 self.cap <= isize::MAX, 所以不会溢出
            // Layout::array 检查是否小于 usize::MAX,
            let new_cap = 2 * self.cap;
            let new_layout = Layout::array::<T>(new_cap).unwrap();
            (new_cap, new_layout)
        };

        assert!(new_layout.size() <= isize::MAX as usize, "Allocation too large");

        let new_ptr = if self.cap == 0 {
            unsafe {alloc::alloc(new_layout)}
        }else{
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe{ alloc::realloc(old_ptr, old_layout, new_layout.size())}
        };

        // 分配失败, `new_ptr` 成为空指针
        self.ptr = match NonNull::new(new_ptr as *mut T){
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.cap = new_cap;
    }
}