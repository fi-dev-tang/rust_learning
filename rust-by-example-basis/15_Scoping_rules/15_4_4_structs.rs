// 结构体中的生命周期定义
/*
结构体里面定义生命周期的用意：
下面的 &'a i32 和 x: &'a i32, y: &'a i32
表示引用至少和 Borrowed , 引用至少和 NamedBorrowed 的生命周期一样长

表示成员变量的数据引用在离开作用域之前，始终是有效的
*/
#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

#[derive(Debug)]
struct NamedBorrowed<'a>{
    x: &'a i32,
    y: &'a i32,
}

#[derive(Debug)]
enum Either<'a>{
    Num(i32),
    Ref(&'a i32),
}

fn main(){
    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NamedBorrowed {x: &x, y: &y};
    let reference = Either::Ref(&x);
    let number = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is borrowed in {:?}", number);
}