/*
在 all_pattern2.rs 里面写了一个非常丑陋的函数
其核心代码应该就是下面的
match(setting_value, new_setting_value) {
    (Some(_), Some(_)) => {
        println!("Can't assign to already customized value");
    },
    _ => {
        setting_value = new_setting_value;
    },
}
println!("setting_value is {:?}", setting_value);

在把上面的核心代码转换成函数匹配之后，遇到了编译过程中的许多问题
1. 所有权的问题

Option<T> 没有实现 Copy trait, 所以会出现 move 的问题，导致旧变量名指向的内容过时，不能使用。

应该在 fn customized_change(setting_value: Option<i32>, new_setting_value: Option<i32>){}
先将函数参数的类型定义为 mutable
语法为 fn function(mut 函数名: Type)

2. 不可变引用不能直接赋值给可变引用

编译器运行的结果: setting_value = new_setting_value; // types differ in mutability 
对于赋值的第二个参数 new_seting_value, 也需要将参数类型修改为 mutable
*/

// 解决所有权的问题，涉及到 setting_value 的部分带上 mutable
// 解决不可变引用赋值给可变引用的问题，new_setting_value 也需要修改成 mutable 类型

fn customized_change1<'a>(mut setting_value:&'a mut Option<i32>, new_setting_value: &'a mut Option<i32>){
    println!("[1] before customized, setting_value = {:?}", setting_value);
    match(setting_value, new_setting_value.clone()){
        (Some(_), Some(_)) => {
            println!("[1] Can't assign to already customized value");
        },
        _ => {
            setting_value = new_setting_value;
            println!("[1] setting_value = {:?}", setting_value);
        },
    }
}

/*
解决前两个问题之后，依然产生报错
3. 生命周期的问题
编译器运行结果:
fn customized_change1(mut setting_value: &mut Option<i32>, mut new_setting_value: &mut Option<i32>)
                                         |                                         - let's call the lifetime of this reference `'1`
                                         let's call the lifetime of this reference `'2`
setting_value = new_setting_value; 
assignment requires that `'1` must outlive `'2`
说明在赋值时，new_setting_value 的生命周期必须比 setting_value 的生命周期要长
编译器说明: 需要对函数增加生命周期的限制(暂时先照做)

4. 作用域的问题
match 进行模式匹配时，会转移变量的所有权
导致 match(setting_value, new_setting_value) 
和 setting_value = new_setting_value 处冲突？

在 match 表达式中，setting_value 发生了移动。
这是因为当元组 (setting_value, new_setting_value) 被创建时，作为匹配的一部分，实际上 "消费" 了这两个引用。

之后再次借用 setting_value, 这是不被允许的，因为 setting_value 已经因“移动”而不认为是有效的引用了。

5. 直接替换引用不被允许
编译器指出的 setting_value = new_setting_value;
实际上直接替换引用 setting_value 本身
Rust 的引用是不可变的绑定到初始值，不能重新绑定到另一个引用。【看成试图修改不可变指针】
*/
fn customized_change2<'a>(mut setting_value: &'a mut Option<i32>, new_setting_value: &'a mut Option<i32>){
    println!("[2] before customized, setting_value: {:?}", setting_value);
    match (setting_value.clone(), new_setting_value.clone()) {
        (Some(_), Some(_)) => {
            println!("[2] can't change already customized value");
        },
        _ => {
            setting_value = new_setting_value;
            println!("[2] after customized, setting_value: {:?}", setting_value);
        }
    }
}

use std::mem;
// 使用 std::mem, 替换引用所指向的内容，不是改不可变指针
// match(setting_value, new_setting_value) 
// 编译器的 move 非常有误导性
fn customized_change3(setting_value: &mut Option<i32>, new_setting_value: &mut Option<i32>){
    println!("[3] before customized, setting_value: {:?}", setting_value);
    match(setting_value.clone(), new_setting_value.clone()){
        (Some(_), Some(_)) => {
            println!("[3] can't change already customized value");
        },
        _ => {
            *setting_value = mem::replace(new_setting_value, None);
        },
    }
    println!("[3] after customized, setting_value: {:?}", setting_value);
}


fn construct_some_type(){
    let mut old_value1 = Some(1i32);
    let mut new_value1 = Some(2i32);
    let mut old_value2 : Option<i32> = None;

    // 改变 1.2. (所有权 && 不可变引用)
    customized_change1(&mut old_value1, &mut new_value1);
    customized_change1(&mut old_value2, &mut new_value1);

    // 改变 3. 生命周期设定 (new_setting_value 的生命周期必须长于 setting_value 的生命周期) -> 直接修改不可变指针会出问题，应该改内容
    customized_change2(&mut old_value1, &mut new_value1);
    customized_change2(&mut old_value2, &mut new_value1);

    // 改变 4. 修改指针的指向
    customized_change3(&mut old_value1, &mut new_value1);
    customized_change3(&mut old_value2, &mut new_value1);
}

/*
一个新的版本:
发现如果在传递函数参数的时候，没有使用引用，语法相对而言简单很多
*/
fn update_some<'a>(mut setting_value:Option<&'a str>, new_seting_value:Option<&'a str>){
    match(setting_value, new_seting_value){
        (Some(_), Some(_)) => {
            println!("Can't assign to customized value");
        },
        _ => {
            setting_value = new_seting_value;
        },
    }
    println!("setting_value = {:?}", setting_value);
}

fn construct_some_str(){
    let old_value1 = Some("hello_string");
    let old_value2: Option<&str> = None;
    let new_value =  Some("Construct string");

    update_some(old_value1, new_value);
    update_some(old_value2, new_value);
}


// 尝试几种核心代码修改的写法
fn main(){
    construct_some_type();     // 1. 只改掉函数参数的 mutable 属性  2. 改掉两部分的 mutable 属性 3. 添加生命周期说明 
    construct_some_str();      // 比较原始的写法，针对 Option<&str>
}