// 自己动手为 Vec<T> 实现 Drop 特性 1. pop(取决于元素是否实现 Drop 特性) 2. dealloc
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