/*
旧写法的问题: 无法直接用 match 表达式来比较或交换两个引用的所有权或绑定
*/
fn customized_change1<'a>(mut setting_value: &'a mut Option<i32>, new_setting_value: &'a mut Option<i32>){
    println!("before change, setting_value: {:?}, new_setting_value: {:?}", setting_value, new_setting_value);
    match(setting_value, new_setting_value.clone()){
        /*
        这里加上 new_setting_value.clone() 的原因:
        match 的作用: 匹配和解构枚举、元组等值，不是直接用于比较或交换引用。
        1. match(setting_value, new_setting_value) 的表达式开始处，尝试匹配, 导致一个值被 "移动"
        2. setting_value = new_setting_value; // match 表达式的分支中尝试再次借用 new_setting_value
        */
        (Some(_), Some(_)) => {
            println!("Can't assign to already customized value");
        },
        _ => {
            setting_value = new_setting_value;
            println!("after change, setting_value: {:?}", setting_value);
            /* 
            这里如果写 println!("after change, setting_value: {:?}, new_setting_value: {:?}", setting_value, new_setting_value);
            会导致出错，出错原因: 
            1. 上面对可变引用赋值时，new_setting_value 作为可变引用 2. 打印时, new_setting_value 退化为不可变引用
            违反了同一作用域内，不能同时存储对一个变量的可变引用和不可变引用
            */
        },
    }
}

fn main(){
    let mut old_value1 : Option<i32> = Some(1i32);
    let mut new_value1 : Option<i32> = Some(2i32);

    let mut old_value2 : Option<i32> = None;
    let mut new_value2 : Option<i32> = Some(4i32);

    customized_change1(&mut old_value1, &mut new_value1);
    customized_change1(&mut old_value2, &mut new_value2);
}