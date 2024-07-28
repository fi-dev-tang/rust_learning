impl<T> Vec<T>{
    fn push(&mut self, elem: T){
        if self.len == self.cap{
            self.grow();
        }

        unsafe{
            ptr::write(self.ptr.as_ptr().add(self.len), elem);
        }
        self.len += 1;
    }
}

impl<T> Vec<T>{
    fn pop(&mut self) -> Option<T>{
        if self.len == 0 {
            None
        }else{
            self.len -= 1;
            Some(ptr::read(self.ptr.as_ptr().add(self.len)))
        }
    }
}