/*
[写在最前]:
认为这次的推理过程比较顺理成章，
因为: variables 在 rust 中不仅负责保存数据，还负责管理和数据相关的所有资源 resources(具体表现在离开作用域时释放资源), 所以每个变量只能有一个所有者

[Ownership & moves]
Because variables are in charge of freeing their own resources, resources can only have one owner.
This also prevents resources from being freed more than once.
Note that not all variables own resources(references)
the ownership is transferred because:
1. doing assignments ( let x = y;)
2. passing function arguments by value
called move
avoids creating dangling pointers

[take-away]
1. 因为变量负责管理资源的释放，所以每个变量都只能有一个所有者
2. 对于 ref 类型的变量，实际上并不拥有资源
3. 当进行赋值，或者函数赋值的时候，变量的所有权需要被转移 move,否则会造成悬垂指针 dangling pointer
*/
fn destory_box(c: Box<i32>){
    println!("Destorying a box that contains {}", c);
}

fn main(){
    let x = 5u32;
    let y = x;
    println!("x is {}, and y is {}", x, y);
    
    let a = Box::new(5i32);
    println!("a contains: {}", a);

    let b = a;
    destory_box(b);
}