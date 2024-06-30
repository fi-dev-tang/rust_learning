/*
要求: 为结构体List 实现打印,
打印的格式为: [0:1, 1:2, 2:3]

? 操作符是一个错误传播运算符，仅能在返回 Result 或 Option 类型的表达式后使用。
当 ? 用于 Result 类型时，如果结果是 Ok(value), 就会返回 value, 继续执行后续代码；
如果是 Err(error), 则会立即从当前函数返回这个错误，同时中断当前函数的执行。

好处：简洁地处理错误，而不需要显式地写出每一个匹配分支（不需使用 match 或 if let 来检查并处理错误）。
*/
use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        // 主要考虑获取元组结构体 List 的成员索引，以及使用 write()? 处理 fmt::Result 返回 Errors 的情况
        let vec = &self.0;

        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", count, v)?;
        }
        write!(f, "]")
    }
}

fn main(){
    let list = List(vec![1,2,3]);
    println!("{}", list);
}