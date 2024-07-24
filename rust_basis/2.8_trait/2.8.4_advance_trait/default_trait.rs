/*
默认类型参数的示例
标准库里关于 Add 特性的写法

pub trait Add<RHS=Self>{    // 定义了一个泛型，泛型的默认参数是 Self, 也就是默认和自身是同一个类型
    type Output;
    fn add(self, rhs: RHS) -> Self::Output;
}
*/
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point{
    x: i32,
    y: i32,
}

impl Add for Point{
    type Output = Point;
    fn add(self, other: Point) -> Point{
        Point{
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// 默认加法特性中相关的定义和实现注意点
fn default_adding_trait(){
    let point1 = Point{x: 2, y: 3};
    let point2 = Point{x: 4, y: 5};

    let point3 = point1 + point2;
    println!("{:?}", point3);
    assert_eq!(point3, Point{x: 6, y: 8});
}

// Add 对于不同类型的泛型相加
#[derive(Debug)]
struct Millimeters(i32);
#[derive(Debug)]
struct Meters(i32);

impl Add<Meters> for Millimeters{
    type Output = Millimeters;
    fn add(self, other: Meters) -> Self{
        Millimeters(self.0 + (other.0) * 1000)
    }
}

fn adding_two_types_together(){
    let millimeters = Millimeters(20);
    let meters = Meters(15);

    println!("{:?}", millimeters + meters);
}

// 同名方法的问题，在不同的特征中，在本类型的方法里，都定义了同名方法
struct Human;

trait Pilot{
    fn fly(&self);
}

trait Wizard{
    fn fly(&self);
}

impl Pilot for Human{
    fn fly(&self){
        println!("[Pilot] calling fly!");
    }
}

impl Wizard for Human{
    fn fly(&self){
        println!("[Wizard] calling fly!");
    }
}

impl Human{
    fn fly(&self){
        println!("[Human self] calling fly!");
    }
}

fn same_function_name(){    // 有 &self 作为参数
    let human = Human;
    human.fly();
    Pilot::fly(&human);
    Wizard::fly(&human);
}

// 关联函数的同名情况处理
struct Dog;

trait Animal{
    fn baby_name() -> String;
}

impl Dog{
    fn baby_name() -> String{
        String::from("Dog self")
    }
}

impl Animal for Dog{
    fn baby_name() -> String{
        String::from("Animal call dog")
    }
}

fn related_function_with_same_name(){
    println!("{}", Dog::baby_name());
    println!("{}", <Dog as Animal>::baby_name());   // 完全限定语法
}

// 多种特征实现，特征定义中的特征约束
use std::fmt::Display;
use std::fmt;

trait Outputline : Display{
    fn output_line(&self){
        let result = self.to_string();
        let len = result.len();

        println!("{}", "*".repeat(len + 4));        // **************************
        println!("*{}*", " ".repeat(len + 2));      // *                        *
        println!("* {} *", result);                 // *     real string        *
        println!("*{}*", " ".repeat(len + 2));      // *                        *
        println!("{}", "*".repeat(len + 4));        // **************************
    }
}

// 上面实现过 point
impl Outputline for Point{} // 这里用默认实现就行，但是 to_string() 的实现是通过 std::fmt::Display 特征中定义的方法来完成，需要手动实现

impl Display for Point{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "Point: ({}, {})", self.x, self. y)
    }
}

// 连续为 Point 实现 Outputline 和 Display 特性
fn trait_bound_in_trait_definition(){
    let point = Point{x: 21, y: 38};
    point.output_line();
}

// 破除孤儿规则: 孤儿规则，如果当前的类型和特征都不是在本作用域中出现，则无法为该类型实现特征
// 一种解决方法，在需要实现的类型外面包裹一层，将其作为新结构体的成员，调用方式为 self.0, 构成元组结构体
// 示例: 为 Vec<String> 重新定义 Display 特征方法
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "[{}]", self.0.join(","))
    }
}

fn implement_display_for_vec_string(){
    let wrapper = Wrapper(vec![String::from("hi"), String::from("close"), String::from("Rust"), String::from("close again")]);
    // println!("{}", wrapper.fmt().unwrap());     // 这种写法是错的.
    /*
    修正一个观念上的错误，fmt 方法是为格式化器 fmt::Formatter 准备的，
    它应该由格式化宏(println!, fmt!) 内部自动调用，而不是直接手动调用， 手动调用的写法 wrapper.fmt().unwrap() 有问题
    */
    println!("{}", wrapper);
}

fn main(){
    default_adding_trait();                 // <RHS=Self> 加法运算符重载
    adding_two_types_together();            // meters + millimeters

    same_function_name();                   // 多种同名方法, 有 &self 作为参数
    related_function_with_same_name();      // 同名关联函数

    // 特征定义中的特征约束
    trait_bound_in_trait_definition();      // 需要预先实现要求的特征

    // 孤儿规则下实现: 在当前作用域中实现不在本作用域中的类型和特征, Wrapper 进行包裹
    implement_display_for_vec_string();
}