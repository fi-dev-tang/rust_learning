/*
引用: 消除函数引用过程中重复改变所有权的问题
*/
fn borrowing_using(){
    let x = 5;
    let y = &x;

    assert_eq!(5, *y);
    // assert_eq!(5, y); // Error! No implementation for `{integer} == &{integer}`
}

/*
引用: 不改变所有权
*/
fn borrowing_string(){
    let s1 = String::from("this is a string");
    let len = takes_string_borrowing(&s1);

    println!("{}'s length = {}", s1, len);
}

fn takes_string_borrowing(a_string: &String) -> usize {
    a_string.len()
}

/*
尝试修改引用的变量
fn _try_change_borrowing(){
    let s1 = String::from("Helo, ");
    change_string_borrowing(&s1);

    println!("try_change_borrowing: {}", s1);
    // Error: some_string is a & reference, so the data it refers to cannot be borrowed as mutable
}


fn change_string_borrowing(some_string: &String){
    some_string.push_str(" , world!");
}
*/
/*
错误原因: 不可变变量的引用，是不能修改原变量的值的
将原变量的类型改为 mutable, 原变量的引用也声明为 mutable reference
*/
fn try_change_mutable_borrowing(){
    let mut s1 = String::from("helo, this");
    change_mutable_string_borrowing(&mut s1);

    println!("try_change_mutable_borrowing: {}", s1);
}

fn change_mutable_string_borrowing(some_string: &mut String){
    some_string.push_str(" is a world!!!");
}

/*
可变引用的限制条件: 同一作用域下只能有一个可变引用
let b_string = &mut a_string;
let c_string = &mut a_string; 
导致错误
*/
fn mutable_borrowing(){
    let mut a_string = String::from("a_string");

    let b_string = &mut a_string;
    //let c_string = &mut a_string;

    println!("b_string: {}", b_string);
    //println!("c_string:{}", c_string);
}

/*
可变引用:
同一作用域下只能有一个可变引用
通过添加大括号，限制可变引用的作用域范围
*/
fn limit_multi_borrowing(){
    let mut s = String::from("limit_multi_borrowing");

    {
        let s1 = &mut s;
        println!("borrowed s1: {}", s1);
    }

    let s2 = &mut s;
    println!("borrowed s2: {}", s2);
}

/*
可变引用与不可变引用不能同时存在
*/
fn mutable_and_non_mutable_coexist(){
    let s = String::from("mutable and non mutable borrowing");

    let r1 = &s;  // 不可变引用
    let r2 = &s;  // 不可变引用

    // let r3 = &mut s; // 可变引用
                     // cannot borrow `s` as mutable because it is also borrowed as immutable

    println!("r1 = {}, r2 = {}", r1, r2);
}

/*
引用的作用域优化：
并不是变量的作用域，必须在某个花括号之后
引用的作用域可以提到不再使用这个变量的位置
*/
fn new_compiler_range(){
    let mut s = String::from("new_compiler_range");
    let r1 = &s;
    let r2 = &s;

    println!("r1 = {}, r2 = {}", r1, r2); // 理论上, r1 和 r2 的作用域在这里结束了

    let r3 = &mut s;
    r3.push_str(" : test if successful, output this line!");
    println!("r3 = {}", r3);
}


/*
悬垂引用(Dangling reference)
指针所指向的内容被释放之后，继续使用指针
*/
fn dangling_reference_example(){
    let dangling_reference = create_non_dangling_reference();
    println!("dangling_reference: {}", dangling_reference);
}

fn create_non_dangling_reference() -> String{  
    /* 
    报错: expected named lifetime parameter
    this function's return type contains a borrowed value, but there is no value for it to be borrowed from
    */
    let a_string = String::from("create_non_dangling_reference");
    a_string
}

fn main(){
    borrowing_using();

    borrowing_string();

    // _try_change_borrowing();
    try_change_mutable_borrowing();

    mutable_borrowing();
    limit_multi_borrowing();

    mutable_and_non_mutable_coexist();  // 可变引用和不可变引用不能同时存在

    new_compiler_range();       // 可变引用和不可变引用不能同时存在, 前提是: 作用域已经结束了

    dangling_reference_example();
}