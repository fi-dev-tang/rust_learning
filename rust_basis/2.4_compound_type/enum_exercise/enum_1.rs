// 1. 在创建枚举时，你可以使用显式的整数设定枚举成员的值
// 修复错误
/*
对照答案修改版本: 理解如下: 1. enum 枚举类型可以按照 C 语言中，设定一个给定值
                          2. as 的用法并不是作为方法调用 .as(), 而是直接写成 Number::One as u8
*/
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
    Zero = 0,
    One = 1,
    Two = 2,
}

fn main(){
    // 通过 `as` 可以将枚举值强转为整数类型
    assert_eq!(Number::One as u8, Number1::One as u8);
    assert_eq!(Number1::One as u8, Number2::One as u8);
}