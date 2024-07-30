/*
实现 Vec<T> 中的 insert 和 remove 功能
具体介绍实现情况: 
1. insert 是在当前的 index 中插入一个元素，
导致原本的 [index .. len] 被移动到 [index + 1 .. len + 1], 这其中一共有 (len - index) 个元素需要被移动
可以这样理解，原本有 len 个元素，在 index 之前一共有 [0..index - 1] 共 index 个元素，所以剩余 len - index 个元素需要移动
首先进行 ptr::copy, 之后再进行 ptr::write 操作

2. remove 是在当前的 index 中删除元素
需要将原来的 [index + 1 .. len + 1] 移动到 [index .. len]
原先有 index + 1 个元素不需要移动，实际上 一共只有 len - index - 1 个元素需要移动
可以先执行 len - 1 的操作
先读取元素，再进行 ptr::copy
*/
use std::ptr;
impl<T> Vec<T>{
    fn insert(&mut self, index: usize, elem: T){
        // 第一步，判断插入的位置是否合法, 如果是 self.len 处当作 push() 操作，也合理
        assert!(index <= self.len, "insert index out of bounds");

        if self.len == self.cap{ self.grow();}          // 控制字符串增长
        unsafe{
            ptr::copy(
                self.ptr.as_ptr().add(index),           // src
                self.ptr.as_ptr().add(index + 1),       // dst
                self.len - index,                       // copy length
            );

            ptr::write(self.ptr.as_ptr().add(index), elem);
        }
        self.len += 1;
    }
}

impl<T> Vec<T>{
    fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len, "remove out of bounds!");

        unsafe{
            self.len -= 1;

            let resut = ptr::read(self.ptr.as_ptr().add(index));
            ptr::copy(
                self.ptr.as_ptr().add(index + 1),       //  [index .. len] <-  [index + 1 .. len + 1]
                self.ptr.as_ptr().add(index),
                self.len - index,
            );
            result
        }
    }
}