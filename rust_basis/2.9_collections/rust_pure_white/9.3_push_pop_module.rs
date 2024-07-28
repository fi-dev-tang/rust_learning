/*
几个要解释的重点:
1. push 操作不能直接 解引用内存
    (原因 1. 内容可能未分配，垃圾值按 T 格式解引用会出问题
          2. 有可能调用 drop 特性对位置上的元素进行析构，垃圾值或者不具备所有权导致出错)
2. pop 操作不能直接出值
    导致元素的 move, 失去所有权，引起未初始化内存，后续如果 push 到这个地点以类型 T 的格式解析出错
*/
impl<T> Vec<T>{
    pub fn push(&mut self, elem: T) {
        if self.cap == self.len + 1 {
            self.grow();
        }

        unsafe{
            ptr::write(self.ptr.as_ptr().add(self.len), elem);
        }
        self.len += 1;
    }
}

impl<T> Vec<T>{
    pub fn pop(&mut self) -> Option<T>{
        if self.len == 0 {
            None
        }else{
            self.len -= 1;
            Some(ptr::read(self.ptr.as_ptr().add(self.len)))
        }
    }
}