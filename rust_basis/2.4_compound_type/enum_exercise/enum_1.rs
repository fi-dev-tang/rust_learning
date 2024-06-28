// 1. 在创建枚举时，你可以使用显式的整数设定枚举成员的值
// 修复错误
#[derive(Debug)]
enum Number{
    Zero,
    One,
    Two,
}

#[derive(Debug)]
enum Number1{
    Zero = 0,
    One,
    Two,
}

// C 语言风格的枚举定义
#[derive(Debug)]
enum Number2{
    Zero(f64) = 0.0,
    One(f64) = 1.0,
    Two(f64) = 2.0,
}

fn main(){
    // 通过 `as` 可以将枚举值强转为整数类型
    let number_one = Number::One;
    let number1_one = Number1::One;

    let number2_one = Number2::One;

    println!("{:?}", number_one);
    println!("{:?}", number1_one);
    println!("{:?}", number2_one);
}