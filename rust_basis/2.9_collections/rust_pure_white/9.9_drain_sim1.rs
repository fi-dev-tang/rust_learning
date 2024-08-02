// 第二种 Vec<T> 迭代器的实现方式: drain(区别于 into_iter), 只对 Vec<T> 实现借用，不消耗 Vec<T>
// part 1 最开始的简单定义，引入限制生命周期为相同的 PhantomData
use std::marker::PhantomData;

pub struct Drain<'a, T: 'a>{
    vec: PhantomData<&'a mut Vec<T>>,
    start: *const T,
    end: *const T,
}

impl<'a, T> Iterator for Drain<'a,T>{   
    type Item = T;
    fn next(&mut self) -> Option<T>{
        if self.start == self.end{
            None
        }
    }
}

// part 2, 由于 IntoIter 和 Drain 都有 start 和 end 字段，实际上拥有相同的结构，可以将 start 和 end 字段做进一步单独的抽象
// 这里的 start 和 end 离开所属结构体的字段，生命周期独立是危险的, 用 unsafe 进行包裹
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
                slice.as_ptr().add(slice.len())
            }
        }
    }
}

// part 3. 在添加了 RawValIter 之后，可以更好地对 IntoIter 进行修改，修改如下:
pub struct IntoIter<T>{
    _buf: RawVec<T>,
    iter: RawValIter<T>,
}

impl<T> IntoIterator for Vec<T>{
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T>{
        let iter = RawValIter::new(&self);
        let bud = ptr::read(&self.buf);
        mem::forget(self);

        IntoIter{
            iter,
            _buf: buf,
        }
    }
}

impl<T> Iterator for IntoIter<T>{
    type Item = T;
    fn next(&mut self) -> Option<T> {self.iter.next()}
    fn size_hint(&self) -> (usize, Option<usize>){ self.iter.size_hint()}
}

impl<T> DoubleEndedIterator for IntoIter<T>{
    fn next_back(&mut self) -> Option<T> {self.iter.next_back()}
}

impl<T> for IntoIter<T>{
    fn drop(&mut self){
        for _ in &mut *self {}
    }
}

// part 4. 借用 RawValIter 来完成 Drain 结构的更好设计:
use std::marker::PhantomData;

pub struct Drain<'a, T:'a>{
    vec: PhantomData<&'a mut Vec<T>>,
    iter: RawValIter<T>,
}

impl<'a, T> Iterator for Drain<'a, T>{
    type Item = T;
    fn next(&mut self) -> Option<T> {self.iter.next()}
    fn size_hint(&self) -> (usize, Option<usize>) -> {self.iter.size_hint()}
}

impl<'a, T> DoubleEndedIterator for Drain<'a, T>{
    fn next_back(&mut self) -> Option<T> {self.iter.next_back()}
}

impl<'a, T> Drop for Drain<'a, T>{
    fn drop(&mut self){
        for _ in &mut *self {}
    }
}

impl<T> Vec<T>{
    fn drain(&self) -> Drain<T>{
        let iter = unsafe {RawValIter::new(&self)};
        self.len = 0;
        Drain{
            vec: PhantomData,
            iter: iter,
        }
    } 
}