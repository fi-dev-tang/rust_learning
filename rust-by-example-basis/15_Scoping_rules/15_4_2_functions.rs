/*
[写在最前]: functions
带有生命周期的函数签名具有约束条件:
1. 任何引用都必须有标注的生命周期
2. 任何被返回的引用必须与输入引用具有相同的生命周期，或者称为静态生命周期
如果没有输入而返回引用，会导致返回指向无效数据的引用，会被禁止
*/
fn print_one<'a>(x: &'a i32){
    println!("`print_one`: x is {}", x);
}

// 可变引用也需要加生命周期
fn add_one<'a>(x: &'a mut i32){
    *x += 1;
}

fn print_multi<'a, 'b>(x: &'a i32, y:&'b i32){
    println!("`print_multi`: x is {}, y is {}", x, y);
}

fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 { x }

fn main(){
    let x = 7;
    let y = 9;

    print_one(&x);
    print_multi(&x, &y);

    let z = pass_x(&x, &y);
    print_one(z);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);
}