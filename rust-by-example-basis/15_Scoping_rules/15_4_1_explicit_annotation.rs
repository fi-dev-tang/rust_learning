/*
[写在最前]: 
确保指向的数据仍然有效时，不会超过其生命周期
Explicit annotation
The borrow checker(借用检查器) uses explicit lifetime annotations(显式生命周期标记) to determine how long references should be valid.
Rust requires explicit annotations to determine what the lifetime of a reference should be.
The syntax for explicitly annotating a lifetime uses an apostrophe character as follows:
foo<'a>
// `foo` has a lifetime parameter `'a`
表示 foo 的生命周期不能超过 'a, 显式的生命周期类型标注是 &'a T
foo<'a, 'b>
// `foo`has a lifetime parameter `'a` and `'b`
*/
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32){
    println!("x is {} and y is {}", x, y);
}

fn failed_borrow<'a>(){
    let _x = 12;
}

/*
特别说明:
fn failed_borrow<'a>(){
    let _x = 12;
    let y: &'a i32 = &_x;
}
这里试图创建一个生命周期为 'a 的引用 y, 该引用指向局部变量 _x。然而，这会导致编译错误，因为 _x 是在 failed_borrow 函数内部定义的，
它的生命周期只限于这个函数的作用域内。
而生命周期 'a 被指定为函数的一个参数，意味着它可能需要比 _x 的实际生命周期更长。
_x 在函数结束后就会被销毁，所以不能保证 y 引用在函数外部仍然有效。
_x 的生命周期比 'a 短
*/
fn main(){
    let (four, nine) = (4, 9);

    print_refs(&four, &nine);

    failed_borrow(); 
}