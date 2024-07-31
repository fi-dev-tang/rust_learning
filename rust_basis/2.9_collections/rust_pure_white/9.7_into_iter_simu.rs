/*
为 Vec<T> 手动实现一种迭代器方法: into_iter, 
这种迭代器方法将获取 Vec<T> 的所有权，为了防止在初次调用 into_iter 构造方法时，Vec<T> 离开作用域自动调用 Drop 方法被 drop 掉，
需要手动为 Vec<T> 实现 ManuallyDrop 方法，手动控制其析构函数的调用。

迭代器有两种：next 和双端迭代器(从后往前) next_back
类似 C 语言的双指针，左边的 start 指针正好指向 Vec<T> 期望迭代的元素，右边的 end 指针(按照从左往右的看法)正好指向 Vec<T> 期望迭代元素的后一个。
图示说明:
        S E
X X X X O X X X X  (S != E , 还有一个元素未被迭代)
总共实现的函数列表:
1. into_iter()
2. next()
3. next_back()
4. size_hint()
5. drop()
*/
use std::ptr::NonNull;
use std::ptr;

pub struct IntoIter<T>{
    buf: NonNull<T>,
    cap: usize,
    start: *const T,
    end: *const T,
}

impl<T> IntoIterator for Vec<T>{
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T>{
        let vec = ManuallyDrop::new(self);  // 手动实现 ManuallyDrop 方法，控制析构函数的调用时机

        let ptr = vec.ptr;     // 错误1: 这里 vec = ManuallyDrop::new(self) 已经获得了所有权，后续不能使用 self, 应该用 vec
        let len = vec.len;
        let cap = vec.cap;

        IntoIter{
            buf: ptr,
            cap: cap,
            start: ptr.as_ptr(),
            end: if cap == 0 {
                // 这里是说还没有分配内存，不能通过这个指针获取偏移
                ptr.as_ptr()
            }else{
                unsafe {ptr.as_ptr().add(len) }
            },
        }
    }
}

impl<T> Iterator for IntoIter<T>{
    type Item = T;
    fn next(&mut self) -> Option<T>{
        if self.start == self.end {
            None
        }else{
            unsafe{
                let result = ptr::read(self.start);
                self.start = self.start.offset(1);
                Some(result)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>){
        let len = (self.end as usize - self.start as usize) / mem::size_of::<T>();
        (len, Some(len))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T>{
    type Item = T;
    fn next_back(&mut self) -> Option<T>{
        if self.start == self.end {
            None 
        }else{
            unsafe{
                self.end = self.end.offset(-1);
                Some(ptr::read(self.end))
            }
        }
    }
}

impl<T> Drop for IntoIter<T>{
    fn drop(&mut self){
        if self.cap != 0 {
            for _ in &mut *self {}
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe{
                alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}