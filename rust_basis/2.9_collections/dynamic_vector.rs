// vector 相关的动态数组操作 (集合类型的重点 1)
// 创建的两种方式: vec![] 或者 Vec::new()
fn vector_create(){
    // 创建:
    let mut array1 : Vec<i32> = Vec::new();
    array1.push(1);

    println!("{:?}", array1);

    let array2 = vec![2, 3, 4];
    println!("{:?}", array2);
}

// 数组访问的方式, 索引 &v[2] 或者具有轻微性能损耗，但以 Option<&T> 返回，非常安全的 .get(2)
fn vector_visit(){
    let v = vec![1, 2, 3, 4, 5];
    let number_3 : &i32 = &v[2];
    println!("number_3 : {}", number_3);

    match v.get(2) {
        Some(third) => println!("third element is {}", third),
        None => println!("Found nothing"),
    }
}

fn vector_usage_ownership(){
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];                          // immutable borrow occurs here, 说明这里是对数组做一个读操作，不可变借用
    v.push(6);                                  // mutable borrow occurs here, 可变借用，对数组进行写
    // println!("first element = {}", first);      // 如果 immutable 只执行一行，在 mutable borrow 之后作用解除，则不会报错，否则会发生报错
}

fn main(){
    vector_create();            // 动态数组的创建与更新
    vector_visit();             // 数组访问 &v[2] 或者 v.get(2)
    vector_usage_ownership();   // 一个简单的数组访问，涉及到 rust 所有权的问题
}