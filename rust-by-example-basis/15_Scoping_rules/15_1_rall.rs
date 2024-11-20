/*
写在最前: [Scoping rules]
Scopes play an important part in ownership, borrowing, and lifetimes.
That is, they indicate to the compiler when borrows are valid, when resources can be freed, and when variables are created or destoryed.

介绍一种 RAII 的方法:(Resource Acquisition is Initialization)
资源的管理（内存、文件句柄、网络连接等）与对象的生命周期绑定在一起 => 作用域的结束标志着对象的生命周期终结

1. variables do more than just hold data, also own resources,
Box<T> owns memory in heap
2. whenever an object goes out of scope, its destructor is called and its owned resources are freed.
*/
fn create_box(){
    let _box1 = Box::new(3i32);
}

fn main(){
    let _box2 = Box::new(5i32);

    {
        let _box3 = Box::new(4i32);
    }

    for _ in 0u32..1_000 {
        create_box();
    }
}