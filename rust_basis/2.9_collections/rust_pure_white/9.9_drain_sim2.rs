// 实现 Drain 迭代器的完整过程和 4 个步骤，drain迭代器移除 Vec<T> 中的所有元素，并返回一个迭代器，
// 注意,为避免 Drain 被 forget 之后，Vec<T> 内存泄漏，令 vec.len = 0
// 步骤一: 直接设计一个 Drain 的结构
use std::marker::PhantomData;

pub struct Drain<'a, T:'a>{
    vec: PhantomData<&'a mut Vec<T>>,
    start: *const T,
    end: *const T,
}

impl<'a, T> Iterator for Drain<'a, T>{
    type Item = T;
    fn next(&mut self) -> Option<T>{
        if self.start == self.end {
            None
        }
    }
}

// 步骤二: 发现 Drain 结构体中的 start 和 end 成员与 IntoIter 中类似，进行第二步抽象，包裹 {start, end} 字段
struct RawValIter<T>{
    start: *const T,
    end: *const T,
}

impl<T> RawValIter<T>{
    unsafe fn new(slice: &[T]) -> Self{
        RawValIter{
            start: slice.as_ptr(),
            end: if slice.len() == 0 {
                slice.as_ptr()
            }else{
                slice.as_ptr.add(slice.len())
            }
        }
    }
}

// 步骤三：有了这个抽象简化，重新实现 IntoIter 中的结构
struct IntoIter<T>{
    _buf: RawVec<T>,
    iter: RawValIter<T>,
}

impl<T> IntoIterator for Vec<T>{
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T>{
        unsafe{
            let buf = ptr::read(&self.buf);
            let iter = RawValIter::new(&self);
            mem::forget(self);

            IntoIter{
                _buf: buf,
                iter: iter,
            }
        }
    }
}

impl<T> Iterator for IntoIter<T>{
    type Item = T;
    fn next(&mut self) -> Option<T> {self.iter.next()}
    fn size_hint(&self) -> (usize, Option<usize>) {self.iter.size_hint()}
}

impl<T> DoubleEndedIterator for IntoIter<T>{
    fn next_back(&self) -> Option<T> {self.iter.next_back()}
}

impl<T> Drop for IntoIter<T>{
    fn drop(&mut self){
        for _ in &mut *self {}
    }
}

// 步骤四: 有了这个抽象，简化 Drain 的结构
pub struct Drain<'a, T:'a>{
    vec: PhantomData<&'a mut Vec<T>>,
    iter: RawValIter<T>,
}

impl<T> Vec<T>{
    fn drain(&mut self) -> Drain<T>{
        let iter = unsafe {RawValIter::new(&self)};
        self.len = 0;

        Drain{
            vec: PhantomData,
            iter: iter,
        }
    } 
}

impl<'a, T> Iterator for Drain<'a, T>{
    fn next(&mut self) -> Option<T> {self.iter.next()}
    fn size_hint(&mut self) -> (usize, Option<usize>) {self.iter.size_hint()}
}

impl<'a, T> DoubleEndedIterator for Drain<'a, T>{
    fn next_back(&mut self) -> Option<T> {self.iter.next_back()}
}

impl<'a, T> Drop for Drain<'a, T>{
    fn drop(&mut self){
        for _ in &mut *self {}
    }
}