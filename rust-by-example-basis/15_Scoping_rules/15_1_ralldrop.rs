/*
展示在变量离开作用域范围之后，自动调用 Drop trait 的实现
大部分不需要手动实现，这里进行一个手动实现的检测
*/
struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self){
        println!("ToDrop is being dropped");
    }
}

fn main(){
    let x = ToDrop;
    println!("Made a ToDrop!");
}