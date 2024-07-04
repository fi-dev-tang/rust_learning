/*
Activity
1. Add a function rect_area which calculates the area of a Rectangle(try using nested destructuring)
2. Add a function square which takes a Point and f32 as arguments, and returns a Rectangle with its top left corner on the point,
and a width and height corresponding to the f32.
*/
/*
第一遍写：打印结果异常诡异，怀疑是精度问题，
rect1 = Rectangle { top_left: Point { x: 10.3, y: 5.2 }, bottom_right: Point { x: 15.9, y: 3.1 } }
The area of rect1 is 11.759998
rect2 = Rectangle { top_left: Point { x: 11.2, y: 15.3 }, bottom_right: Point { x: 16.3, y: 10.200001 } }
The area of rect2 is 26.009995

实际几个涉及到的数据应该是: 11.76, 10.20, 26.01
将全部的 f32 换成 f64(更奇怪了orz)
*/
#![allow(dead_code)]

#[derive(Debug)]
struct Point{
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle{
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rectangle: &Rectangle) -> f32 {
    let (top_x, top_y) = (rectangle.top_left.x, rectangle.top_left.y);
    let (bottom_x, bottom_y) = (rectangle.bottom_right.x, rectangle.bottom_right.y);
    let area = (bottom_x - top_x) * (top_y - bottom_y);
    area 
}

fn square(top: Point, length: f32) -> Rectangle {
    let bottom_right: Point = Point {
        x: top.x + length,
        y: top.y - length,
    };

    let rect: Rectangle = Rectangle{
        top_left: top,
        bottom_right: bottom_right,
    };
    rect
}

fn main(){
    let top_left_1: Point = Point{x: 10.3, y: 5.2};
    let bottom_right_1: Point = Point{x: 15.9, y: 3.1};

    let rect1: Rectangle = Rectangle{
        top_left: top_left_1,
        bottom_right: bottom_right_1,
    };

    println!("rect1 = {:?}", rect1);

    println!("The area of rect1 is {}", rect_area(&rect1));

    let top_left_2: Point = Point{x: 11.2, y: 15.3};
    let length: f32 = 5.1;
    let rect2: Rectangle = square(top_left_2, length);

    println!("rect2 = {:?}", rect2);
    println!("The area of rect2 is {:.2}", rect_area(&rect2));
}