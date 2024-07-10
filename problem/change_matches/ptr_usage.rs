/*
引用修改的例子: 1. 修改指向的值  2. 修改指向
*/
// 修改指向的值
fn change_pointed_value(){
    let mut number: Option<i32> = Some(10i32);
    let ptr: &mut Option<i32> = &mut number;

    println!("ptr's value before {:?}", ptr);
    *ptr = Some(20i32);
    println!("ptr's value after {:?}", ptr);
}

// 修改函数外部的值
fn change_outerside_value(ptr: &mut Option<i32>, value: Option<i32>){
    *ptr = value;
}


// 修改指向
fn change_pointer_direction(){
    let mut number = Some(20i32);
    let mut ptr: &mut Option<i32> = &mut number;
    println!("before change pointer direction: value of {:?}", ptr);    

    let mut number2 = Some(40i32);
    ptr = &mut number2;

    println!("change pointer direction: value of {:?}", ptr);
}

fn main(){
    change_pointed_value();             // 修改可变引用指向的值
    // 修改函数外部的值
    let mut main_number: Option<i32> = Some(60i32);
    let new_value = Some(70i32);
    let main_number_pointer: &mut Option<i32> = &mut main_number;

    change_outerside_value(main_number_pointer, new_value);
    assert_eq!(main_number, new_value);
    println!("main_number {:?}", main_number);

    change_pointer_direction();         // 修改可变引用的指向
}