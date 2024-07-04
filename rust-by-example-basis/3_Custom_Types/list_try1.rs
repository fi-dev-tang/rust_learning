/* 
自己利用 enum 实现List, 并且包含 List 的三个功能
1. 创建一个新的 List
2. 计算 List 的长度
3. 打印完整的 List
4. 在 List 之前添加新的节点
*/
use crate::List::*;

enum List{
    Cons(u32, Box<List>),
    Nil,
}

impl List{
    // 1. 创建新的 List
    fn new() -> List {
        Nil
    }

    // 2. 在链表之前添加一个元素
    fn prepend(self, elem: u32) -> List{
        Cons(elem, Box::new(self))
    }

    // 3. 计算链表的长度
    fn len(&self) -> u32 {
        match *self{
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        } 
    }

    // 4. 打印完整的链表
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main(){
    let mut list: List = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("The length of list is {}", list.len());
    println!("List look like: {}", list.stringify());
}