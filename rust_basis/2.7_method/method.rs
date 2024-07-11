/*
Rust 中定义的方法。
实现数据和方法分离 
struct {data;}     impl {method;}   
*/
#[allow(dead_code)]
struct Circle{
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle{
    // new 是 Circle 的关联函数，因为它的第一个参数不是 self, 且 new 并不是关键字
    // 这种方法往往用于初始化当前结构体的实例
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle{
            x: x,
            y: y,
            radius: radius
        }
    }
    //&self, 借用当前的结构体
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}


fn circle_method(){
    let circ = Circle::new(0.0, 0.0, 1.0);
    println!("the area of circle {}", circ.area());
}

pub struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    pub fn new(width: u32, height: u32) -> Self{
        Rectangle{
            width: width,
            height: height,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 方法名和作用名相同
    fn width(&self) -> bool {
        self.width > 0
    }

    pub fn height(&self) -> u32 {
        return self.height;
    }

    // 带有多个参数的方法
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn rectangle_method(){
    let rec1 = Rectangle{
        width: 1,
        height: 2,
    };

    println!("The area of rec1 is {}", rec1.area());
}

fn same_method_name_and_variable_name(){
    let rect2 = Rectangle{width: 1, height: 3};
    if rect2.width() {
        println!("rect2 has non-zero width: {}", rect2.width);
    }
}

// 方法名和字段名相同 = getter 访问器
fn implement_getter(){
    let rect = Rectangle::new(10, 20);
    println!("Getting rect's height {}", rect.height());
}

fn method_with_multi_parameter(){
    let rect1 = Rectangle::new(10, 20);
    let rect2 = Rectangle::new(20, 30);
    let rect3 = Rectangle::new(30, 20);

    println!("Can rect2 hold rect1 ?:{}", rect2.can_hold(&rect1));
    println!("Can rect2 hold rect3 ?:{}", rect2.can_hold(&rect3));
}

struct Pair(Box<i32>, Box<i32>);

impl Pair{
    fn new(x: i32, y: i32) -> Self {
        Pair(Box::new(x), Box::new(y))
    }

    fn destory(self){   // 这个写法直接获得 self 的所有权
        let Pair(first, second) = self;
        println!("Destorying pair: {}, {}", first, second);
        /*
        其实不加这行打印也ok，
        在之前的提示中，只要 first 和 second 分别带了下划线 _first, _second, 则产生绑定关系，所有权转移
        */
    }   // 在作用域结束后，first 和 second 被销毁
}

/*
一个典型的销毁所有权的例子:
*/
fn exercise_example(){
    let pair = Pair::new(1, 2);
    pair.destory();
}

fn main(){
    circle_method();                            // 实现 circle 并计算面积
    rectangle_method();                         // 实现 rectangle 并计算面积

    same_method_name_and_variable_name();       // 方法名和作用名相同

    implement_getter();                         // 方法名和作用名相同, getter 
    method_with_multi_parameter();              // 带有多个参数的方法 

    exercise_example();                         // 所有权销毁的例子
}