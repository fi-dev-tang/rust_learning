/*
结构体的可见性: 
The visibility only matters when a struct is accessed from outside the module where it is defined,
and has the goal of hiding information(encapsulation).
1. 没有显示声明为 pub 的 struct 中的内容不可以直接进行访问构造
2. 可以定义 impl new 方法等进行访问构造
类似 class 里面的 get, set 方法
*/
mod my{
    pub struct OpenBox<T>{
        pub contents: T,
    }

    pub struct ClosedBox<T>{
        contents: T,
    }

    impl<T> ClosedBox<T>{
        // A public constructor method
        pub fn new(contents: T) -> ClosedBox<T>{
            ClosedBox{
                contents: contents,
            }
        }
    }
}

fn main(){
    // Public structs with public fields can be constructed as usual
    let open_box = my::OpenBox{ contents: "public information"};

    // and their fields can be normally accessed.
    println!("The open box contains: {}", open_box.contents);

    // structs with private fields can be created using public constructors
    let _closed_box = my::ClosedBox::new("classified information");
}