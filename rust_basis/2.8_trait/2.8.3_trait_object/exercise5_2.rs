/* 自己实现修改，将原本不能成为特征安全的返回值 Self , 变成特征安全的返回值 */
// 第一种，限制统一特征类型
fn example1(){
    trait MyTrait{
        fn f(&self) -> Self;    // 第一种实现方案，这里的返回值 Self 就没法用特征对象 Box<dyn MyTrait> 或者 &dyn MyTrait
    }

    impl MyTrait for u32{
        fn f(&self) -> Self{
            42
        }
    }

    impl MyTrait for String{
        fn f(&self) -> Self{
            self.clone()
        }
    }

    // 这里没法用特征对象，只能使用 impl MyTrait 实现
    fn my_function(x: impl MyTrait) -> impl MyTrait{
        x.f()
    }

    let x = 13_u32;
    let y = String::from("Hello Rust!");
    my_function(x);
    my_function(y);
    println!("Success!");
}

// 第二种，修改 Self 的返回值类型
// fn f(&self) -> Self 的返回值类型不能作为特征安全来实现特征对象
fn example2(){
    trait MyTrait{
        fn f(&self) -> Box<dyn MyTrait>;
    }

    impl MyTrait for u32{
        fn f(&self) -> Box<dyn MyTrait>{
            Box::new(42)
        }
    }

    impl MyTrait for String{
        fn f(&self) -> Box<dyn MyTrait>{
            Box::new(self.clone())
        }
    }

    fn my_function(x: Box<dyn MyTrait>) -> Box<dyn MyTrait>{
        x.f()
    }

    let x = Box::new(13_u32);
    let y = Box::new(String::from("Rust Hello!"));
    my_function(x);
    my_function(y);
    println!("Success!");
}

fn main(){
    example1();
    example2();
}