/*
Custom Types 自定义类型
Rust custom data types are formed mainly through the two keywords:
1. struct: define a struct
2. enum: define an enumeration
Constants can also be created via the const and static keywords.
*/
/*
There are three types of structures("structs") that can be created using the struct keyword:
1. Tuple structs, which are basically, named tuples.
2. The classic C structs
3. Unit structs, which are field-less, are useful for generics.
*/
#![allow(dead_code)]

#[derive(Debug)]
struct Person{
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

struct Point{
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
struct Rectangle{
    top_left: Point,
    bottom_right: Point,
}

fn main(){
    let name = String::from("Peter");
    let age = 27;
    let peter = Person {name, age};

    println!("{:?}", peter);

    let point: Point = Point{x: 10.3, y: 0.4};
    let another_point: Point = Point{x: 5.2, y: 0.2};

    println!("point coordinates: ({}, {})", point.x, point.y);

    let bottom_right = Point {x: 5.2, ..another_point};

    println!("second point: ({},{})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point {x: left_edge, y: top_edge} = point;

    let _rectangle = Rectangle {
        top_left: Point {x: left_edge, y: top_edge},
        bottom_right: bottom_right,
    };

    let _unit = Unit;

    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}