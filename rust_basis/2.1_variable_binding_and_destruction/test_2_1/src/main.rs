fn main(){
    let x = define_x();
    println!("{}, world",x);
    test_6();
}

/*
define_x() 中，由于 "hello" 字符串字面量在 Rust 中确实拥有静态生命周期(只放在只读的，全局的数据段)
返回了字符串字面量 "hello" 的引用，编译时确定，其生命周期与整个程序运行周期相同。
x 拥有整个程序运行期间的生命周期。
*/
fn define_x() -> &'static str {
    let x = "hello";
    x
}

fn test_6() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    let x = x;


    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!"; 
}