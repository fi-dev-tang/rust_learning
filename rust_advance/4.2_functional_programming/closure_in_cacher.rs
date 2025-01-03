/*
闭包用在 Cacher 上，
Cacher 作用: 给定一个函数，本地存储一个hash表，当传入一个值的时候，首先判断哈希表中是否有对应的返回值，没有的话再使用原来的函数进行计算
Cacher 结构体 {
        calculation: 传入待记录的函数
        value: HashMap
    }
认为这里的闭包体现的不是很明显，或许可以在实现 Cacher::new() 的时候传递一个闭包作为输入
Cacher::new(|num| {
    println!("Entering expensive_calculation with parameter {}", num);
    thread::sleep(Duration::from_secs(2));
    num * num
});

支持闭包的原因是: 闭包被限制为实现了 Fn(U) -> V trait 的类型
*/
use std::collections::HashMap;
use std::cmp::Eq;
use std::hash::Hash;

use std::thread;
use std::time::Duration;

struct Cacher<T,U,V>
    where T: Fn(U) -> V,
          U: Eq + Hash + Copy,
          V: Copy,
{
    calculation: T,
    value: HashMap<U,V>
}

impl<T,U,V> Cacher<T,U,V>
    where T: Fn(U) -> V,
    U: Eq + Hash + Copy,
    V: Copy,
{
    fn new(calculation: T) -> Cacher<T,U,V>{    // 传入一个函数 T, 把这个函数转换成对应的 Cacher 类型
        Cacher{
            calculation: calculation,
            value: HashMap::new(),
        }
    }

    fn get(&mut self, arg: U) -> V {
        match self.value.get(&arg) {
            Some(v) => *v, 
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

fn expensive_calculation(num: u32) -> u32{
    println!("Entering expensive_calculation with parameter {}", num);
    thread::sleep(Duration::from_secs(2));
    num * num
}

fn main(){
    let mut cacher = Cacher::new(expensive_calculation);
    println!("{}", cacher.get(3));
    println!("{}", cacher.get(3));
    println!("{}", cacher.get(4));
    println!("{}", cacher.get(4));
    println!("{}", cacher.get(5));
}