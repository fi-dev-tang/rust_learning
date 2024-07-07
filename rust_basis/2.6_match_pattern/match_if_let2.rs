/*
理论的部分：matches! 宏
Rust 标准库中提供了一个非常实用的宏: matches!, 它可以将一个表达式跟模式进行匹配，然后返回匹配的结果 true or false
例如，有一个动态数组，里面存有以下枚举:
enum MyEnum {
    Foo,
    Bar
}

fn main(){
    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
}

现在如果想对 v 进行过滤，只保留类型是 MyEnum::Foo 的元素，可以这么写:
v.iter().filter(|x| x == MyEnum::Foo); 
但实际上这行代码会报错，因为你无法将 x 直接跟一个枚举成员进行比较。好在，你可以使用 match 来完成，但是会导致代码更为啰嗦，是否有更简洁的形式？答案是使用 matches!:
v.iter().filter(|x| matches!(x, MyEnum::Foo));  很简单也很简洁，来看看更多的例子:
let foo = 'f'; assert!(matches!(foo, 'A'..='Z' | 'a'...='z'));
let bar = Some(4); assert!(matches!(bar, Some(x) if x > 2));
*/
#[derive(Debug)]
enum MyEnum{
    Foo,
    Bar,
}

// matches! 宏的使用
fn usage_of_matches_macro(){
    // 1. 过滤枚举类型数组中的某些变量
    let vec = [MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    let filter_list : Vec<&MyEnum> = vec.iter().filter(|x| matches!(x, MyEnum::Foo)).collect();
    println!("[MyEnum] filter_list: {:?}", filter_list);

    // 2. 过滤普通的字符串类型
    let table_array = ['a', 'b', 'c', 'A', 'D','f'];
    let filter_table_array : Vec<&char> = table_array.iter().filter(|x| matches!(x, 'a'..='z')).collect();
    println!("[Char] filter_table_array: {:?}", filter_table_array);

    // 3. 过滤 Option 类型的控制
    let _some_array = [Some(1u8), Some(0u8), Some(3u8), Some(2u8), Some(5u8), Some(7u8)];
    // let filter_some_array : Vec<_> = some_array.iter().filter(|x| matches!(x, (Some(i),i > 2u8))).collect();
    // 语法错误！
    // println!("[Some] filter_some_array: {:?}", filter_some_array);

    // 4. assert 的语句判断
    let foo = 'f';
    assert!(matches!(foo, 'a'..='z'|'A'..='Z'));

    let _value = Some(4u8);
    // assert!(matches!(value, Some(i)  i > 2)); 写法错误！
}

// 上面没写对的地方，对枚举类型或 Some 中的元素进行判断
fn matches_assert_value_inside(){
    let some_array = [Some(1u8), Some(2u8), Some(3u8), Some(4u8), Some(5u8), Some(6u8)];
    let filter_some_array : Vec<_> = some_array.iter().filter(|x| matches!(x, Some(i) if i % 2 == 0)).collect();
    println!("[Some]: filter_some_array = {:?}", filter_some_array);

    let value = Some(4u8);
    assert!(matches!(value, Some(i) if i > 2));
}

// if let 中的变量遮蔽，！最好不要使用同名变量
fn shadowing_in_if_let(){
    let age = Some(30);
    println!("Before if let, age = {:?}", age);
    if let Some(age) = age {
        println!("In if let, age = {:?}", age);
    }

    println!("After if let, age = {:?}", age);
}

// match 中的变量遮蔽
fn shadowing_in_match(){
    let age = Some(30);
    match age {
        Some(age) => println!("age here is {:?}", age),
        _ => (),
    };
    println!("after match, age is {:?}", age);
}

fn main(){
    usage_of_matches_macro();       // matches! 作为 filter 过滤的宏的使用
    matches_assert_value_inside();  // matches! 对枚举类型中包含的元素进行判断

    shadowing_in_if_let();          // if let 中的变量遮蔽
    shadowing_in_match();           // match 中的变量遮蔽
}