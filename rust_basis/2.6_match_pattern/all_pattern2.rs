// 解构定长数组
fn destruct_fixed_array(){
    let arr: [u16; 2] = [114, 514];
    let [x, y] = arr;
    println!("x:{}, y:{}", x, y);
    assert_eq!(x, 114);
    assert_eq!(y, 514);
}

// 解构不定长数组
fn destruct_unfixed_array(){
    let arr: &[u16] = &[114, 514];

    if let [x, ..] = arr {
        println!("unfixed_array: x is {:?}", x);
        assert_eq!(x, &114);
    }

    if let &[.., y] = arr {
        println!("unfixed_array: y is {:?}", y);
        assert_eq!(y, 514);
    }

    // 解构空数组
    let empty_arr : &[u16] = &[];
    assert!(matches!(empty_arr, &[..]));
    assert!(!matches!(empty_arr, &[_x, ..]));
}

// 解构数组
fn destruct_array(){
    destruct_fixed_array();
    destruct_unfixed_array();
}

fn foo(_: i32, y: i32){
    println!("the foo function only use y parameter: {}", y);
}

// 使用 _ 忽略函数参数
fn ignore_function_parameter_using_(){  
    foo(3, 4);
}

fn customized_change<'a>(mut setting_value: &'a mut Option<i32>, new_setting_value: &'a mut Option<i32>){
    match (setting_value.clone(), new_setting_value.clone()) {
        (Some(_), Some(_)) => {
            println!("Can't change already existing customized value");
        },
        _ => {
            setting_value = new_setting_value;
        },
    };
    println!("after match, setting_value : {:?}", setting_value);
}

// 使用 _ 忽略部分值
fn ignore_partial_value_using_(){
    let mut old_value1 = Some(5i32);
    let mut old_value2 = Some(10i32);
    let mut old_value3 = None;

    customized_change(&mut old_value1, &mut old_value2);
    customized_change(&mut old_value3, &mut old_value2);
}

fn main(){
    destruct_array();                           // 解构数组
    ignore_function_parameter_using_();         // 使用下划线忽略函数参数
    ignore_partial_value_using_();              // 是用下划线忽略匹配中的部分值
}