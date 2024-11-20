/*
别名
*/
struct Point {x: i32, y: i32, z: i32}

fn main(){
    let mut point = Point{x: 0, y: 0, z: 0};

    {
        let borrowed_point = &point;
        let another_borrow = &point;

        println!("Point has coordinates: ({}, {}, {})",
        borrowed_point.x, another_borrow.y, point.z);
    }

    {
        let mutable_borrow = &mut point;

        // ![重要]: 可以使用 mutable reference, 可变引用修改原数据
        mutable_borrow.x = 5;
        mutable_borrow.y = 2;
        mutable_borrow.z = 1;

        println!("Point has coordinates: ({},{},{})", 
        mutable_borrow.x, mutable_borrow.y, mutable_borrow.z);

    }

    let borrowed_point = &point;
    println!("Point has coordinates: ({},{},{})", 
    borrowed_point.x, borrowed_point.y, borrowed_point.z);
}