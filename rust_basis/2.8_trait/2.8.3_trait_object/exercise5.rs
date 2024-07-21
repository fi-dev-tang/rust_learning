/*
Rust By Practice 特征对象的练习部分
一个特征能变成特征对象，首先该特征必须是对象安全的，即该特征的所有方法都必须拥有以下特点:
1. 返回类型不能是 Self
2. 不能使用泛型参数
题目要求，使用至少两种方法让代码工作，不要删除或添加任何代码行
trait MyTrait{
    fn f(&self) -> Self;
}
impl MyTrait for u32{
    fn f(&self) -> Self {42}
}
impl MyTrait for String{
    fn f(&self) -> Self {self.clone()}
}
fn my_function(x: Box<dyn MyTrait>){
    x.f()
}
fn main(){
    my_function(Box::new(13_u32));
    my_function(Box::new(String::from("abc")));
    println!("Success!")
}
*/
fn exercise5_way1(){
    trait MyTrait{
        fn f(&self) -> Self;
    }

    impl MyTrait for u32{
        fn f(&self) -> u32 { 42}
    }

    impl MyTrait for String{
        fn f(&self) -> String {self.clone()}
    }

    fn my_function(x: impl MyTrait) -> impl MyTrait{
        x.f()
    }

    my_function(13_u32);
    my_function(String::from("abc"));
    println!("exercise5_way1: Success!");
}

fn exercise5_way2(){
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

    my_function(Box::new(13_u32));
    my_function(Box::new(String::from("abc")));
    println!("exercise5_way2: Success!");
}

fn main(){
    exercise5_way1();
    exercise5_way2();
}