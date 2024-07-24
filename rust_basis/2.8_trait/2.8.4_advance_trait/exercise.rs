/* 深入理解特征一节的习题 */
// 练习 2
/*
回顾库中默认泛型类型参数的写法: 
1. 在特征定义的 Add后面增加了泛型参数约束 Add<RHS=Self>
2. 具体实现了两个元组结构体，Add 的 RHS 直接替换成确定类型的元组结构体

trait Add<RHS = Self>{
    type Ouput;
    fn add(self, rhs: RHS) -> Self::Output;
}
impl Add<Meters> for Millimeters{
    type Output = Millimeters;
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
这道题的不同之处,   1. 我们没有重新定义 trait Sub<RHS=Self> 这种写法
                   2. Point<T> 是一个泛型结构体，而不是元组结构体，可以直接赋值给 RHS.
*/
fn exercise_2_method_1(){
    use std::ops::Sub;
    #[derive(Debug, PartialEq)]
    struct Point<T>{
        x: T,
        y: T,
    }

    // 方法一:
    /*
    对这一行进行重点修饰: 
    1. <T: Sub<Output = T>> 属于特征约束(trait bound), 表示 T 必须实现 Sub trait, 并且 Sub trait 的 Output 关联类型也必须是 T
    意味着 T 必须支持减法运算，减法的结果类型也必须是 T。
    2. Sub<Point<T>> Sub 是一个约束，Sub<Point<T>> 表示我们将为 Point<T> 实现 Sub trait
    Sub<Point<T>> 也就是 RHS = Point<T>, 也可以修改为其它类型 Sub<&Point<T>>
     */
    impl<T: Sub<Output = T>> Sub<Point<T>> for Point<T>{
        type Output = Self;

        fn sub(self, other:Self) -> Self::Output{
            Point{
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }

    let point1 = Point{x: 2, y: 3};
    let point2 = Point{x: 1, y: 0};
    let point3 = Point{x: 1, y: 3};
    println!("[exercise 2 method 1]: point1 + point2 = {:?}, point3 = {:?}", point1 - point2, point3);
}

fn exercise_2_method_2(){
    use std::ops::Sub;
    #[derive(Debug, PartialEq)]
    struct Point<T>{
        x: T,
        y: T,
    }

    // 方法二:
    impl<T: Sub<Output = T>> Sub<Self> for Point<T>{
        type Output = Self;

        fn sub(self, other: Self) -> Self::Output{
            Point{
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }

    let point1 = Point{x: 2, y: 3};
    let point2 = Point{x: 1, y: 0};
    let point3 = Point{x: 1, y: 3};
    println!("[exercise 2 method 2]: point1 + point2 = {:?}, point3 = {:?}", point1 - point2, point3);
}

fn exercise_2_method_3(){
    use std::ops::Sub;
    #[derive(Debug, PartialEq)]
    struct Point<T>{
        x: T,
        y: T,
    }

    // 方法三:
    impl<T:Sub<Output = T>> Sub for Point<T>{
        type Output = Self;

        fn sub(self, other:Self) -> Self::Output{
            Point{
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }

    let point1 = Point{x: 2, y: 3};
    let point2 = Point{x: 1, y: 0};
    let point3 = Point{x: 1, y: 3};
    println!("[exercise 2 method 3]: point1 + point2 = {:?}, point3 = {:?}", point1 - point2, point3);
}

// 用三种方法填空，两种使用默认的泛型参数，另外一种不使用
fn exercise_2(){
    exercise_2_method_1();
    exercise_2_method_2();
    exercise_2_method_3();
}

fn main(){
    exercise_2();
}