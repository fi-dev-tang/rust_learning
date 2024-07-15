// 示例 1. const 泛型参数的使用，同时使用泛型参数 T 和 const 泛型参数 N 来描述不同长度，不同类型的数组
use std::fmt::{Debug, Formatter};

struct _ArrayPair<T, const N: usize>{
    left:   [T ; N],
    right:  [T ; N],
}

impl<T: Debug, const N: usize> Debug for _ArrayPair<T, N>{  // 这里写成 impl trait for _ArrayPair<T, N> ; trait 为 Debug
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error>{
        write!(f, "Hello")
    }
}

/*
示例二: 对使用 const 泛型实参的三种情况说明
1. 一个单独的 const 泛型参数
2. 一个字面量值(布尔型，常量, 字符)
3. 一个不含有任何泛型参数的 const 表达式
*/
fn _foo<const N: usize>(){}

fn _bar<T, const M: usize>(){
    _foo::<M>();                                // ok, 第一种情况，一个单独的泛型参数
    _foo::<2021>();                             // ok, 第二种情况，一个字面量值(常量)
    _foo::<{20 * 10 + 1000}>();                 // ok, 第三种情况, 一个不含有任何泛型参数的 const 表达式

    // _foo::<{M + 1}>();                            // error: M + 1 违反第三种情况
    // _foo::<{std::mem::size_of::<T>()}>();        // error: 违反第三种情况 含有泛型参数 T

    let _ : [u8; M];                                // ok, 第一种情况，一个单独的泛型参数
    // let _ : [u8; std::mem::size_of::<T>()];     // error, 第三种情况，泛型参数 
}

/*
示例三: const 泛型运行效率更高
默认设置存储头 head, 以及可以不断增长的 tail, 动态数组
*/
pub struct MinSlice<T, const N: usize>{
    pub head: [T; N],
    pub tail: Vec<T>,
}

impl<T: Copy, const N: usize> MinSlice<T,N>{
    fn from_slice(slice: &[T]) -> Option<Self>{
        if slice.len() < N{
            return None;
        }

        let mut head = [slice[0]; N];
        head.copy_from_slice(&slice[..N]);
        let tail = slice[N..].to_vec();
        Some(MinSlice{head, tail})
    }

    fn unwrap(self) -> Self{
        self
    }
}

fn main(){
    let slice : &[u8] = b"Hello ,world";
    let reference: Option<&u8> = slice.get(6);
    assert!(reference.is_some());

    let slice: &[u8] = b"Hello ,world";
    let minslice = MinSlice::<u8, 12>::from_slice(slice).unwrap();
    let value = minslice.head[6];
    assert_eq!(value, b',');
}