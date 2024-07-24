/*
对标准库 std::ops::Sub 的更新理解
标准库为特征 Sub 实现的定义应该是:
trait Sub<RHS=Self>{
    type Output = Self;
    fn sub(self, other: RHS) -> Self::Output;
}
采用三种方式实现 Sub 特征的实现: 两种显式的默认值修改，一种隐式的实现    
*/
// 第一种为泛型结构体 Point<T> 实现自定义 sub, 默认的 RHS = Point<T>
fn ex2_1(){
    use std::ops::Sub;
    #[derive(Debug)]
    struct Point<T>{
        x: T,
        y: T,
    }

    // 重点: 1. 特征约束 T 必须实现减法 trait, 同时输出的值也要是 T 类型 2. Sub 的 RHS 显式定义为 Point<T>
    impl<T: Sub<Output = T>> Sub<Point<T>> for Point<T>{
        type Output = Self;
        fn sub(self, other: Point<T>) -> Self::Output{
            Point{
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }

    let point1 = Point{x: 12, y: 13};
    let point2 = Point{x: 7, y: 10};
    println!("[1] point1 - point2 = {:?}", point1 - point2);
}

fn ex2_2(){
    use std::ops::Sub;
    #[derive(Debug)]
    struct Point<T>{
        x: T,
        y: T,
    }

    // 重点: 1. 特征约束 T 必须实现减法 trait, 同时输出的值也要是 T 类型 2. Sub 的 RHS 显式定义为 Point<T>
    impl<T: Sub<Output = T>> Sub<Self> for Point<T>{
        type Output = Self;
        fn sub(self, other: Self) -> Self::Output{
            Point{
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }

    let point1 = Point{x: 12, y: 13};
    let point2 = Point{x: 7, y: 10};
    println!("[2] point1 - point2 = {:?}", point1 - point2);
}

fn ex2_3(){
    use std::ops::Sub;
    #[derive(Debug)]
    struct Point<T>{
        x: T,
        y: T,
    }

    // 重点: 1. 特征约束 T 必须实现减法 trait, 同时输出的值也要是 T 类型 2. Sub 的 RHS 显式定义为 Point<T>
    impl<T: Sub<Output = T>> Sub for Point<T>{
        type Output = Self;
        fn sub(self, other: Self) -> Self::Output{
            Point{
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }

    let point1 = Point{x: 12, y: 13};
    let point2 = Point{x: 7, y: 10};
    println!("[3] point1 - point2 = {:?}", point1 - point2);
}

fn main(){
    ex2_1();
    ex2_2();
    ex2_3();
}