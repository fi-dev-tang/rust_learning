fn main(){
    another_function(5,6);
    let x0 = plus_five(5);
    println!("The value of x0 is:{}", x0);

    let x1 = plus_or_minus(5);
    println!("The value of x1 is:{}", x1);

    // exercise_5();
   // _exercise_4_understanding(0);
   _exercise_3();
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is:{}", x);
    println!("The value of y is:{}", y);
}

fn plus_five(x: i32) -> i32 {
    x + 5
}

fn plus_or_minus(x: i32) -> i32 {
    if x > 5 {
        return x - 5
    }
    x + 5
}

/*
使用三种方式实现发散函数
1. 知识点: match 的语法，如果不关心具体值为多少，可以写成 _ => {}
match tp {
    1 => {}
    _ => {}
};

2. 关于发散函数  
never_return_fn() 的至少三种实现方式

(1) fn never_return_fn() -> ! {
    unimplemented!()
}

(2) fn never_return_fn() -> ! {
    panic!()
}

(3) fn never_return_fn() -> ! {
    todo!();
} 

(4) fn never_return_fn() -> ! {
    loop{
        std::thread::sleep(std::time::Duration::from_secs(1))
    }
}
*/
fn _exercise_4(){
    println!("Exercise 4_success!");
}

fn _get_option(tp: u8) -> Option<i32>{
    match tp {
        1 => {

        }
        _ => {

        }
    };

    never_return_fn()
}

fn never_return_fn() -> !{
    unimplemented!()
}


fn _exercise_5(){
    // FILL in the blank
    let b = false;

    let _v = match b {
        true => 1,
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic")
        }
    };

    println!("Exercise6 Failed if printing out this line~");
}


/*
对练习 4 的理解
[自己对于 unimplement!() 后面加不加分号的理解 ]
Note: 其中是否在 panic!() unimplemented!() 或者是 todo!() 后面加分号，
目前自己的理解:
1. 如果不加分号，表明是一个表达式
说明 never_return_fn() 的返回值是这个 永不返回的函数， 实际的效果: 执行 never_return_fn 等价于执行其中的函数，永不返回，达到效果
2. 如果加分号，表示是一个语句
这个语句开始执行上面的程序，永不返回，达到定义的目的

其实就是 todo!() 这个宏即使不加分号，编译器也会当作表达式尝试执行，然后内部包含 panic!() 导致系统永不返回、

发散函数的实际作用: 
一个服务器的守护进程可能是无限循环的，因此是发散函数
函数里部分代码因为各种原因暂时留空不写时，程序会因为缺少对应类型返回值无法通过编译；
如果暂时给返回值类型记作() 到时这些函数需要全部批量修改函数签名会很麻烦
todo!() unimplemented!() 这些宏对应的 !类型可以转换为任意类型，也就规避了这个问题，后期实现真正的业务逻辑时也不需要修改函数签名，删掉宏并替换为自己的代码。
*/
fn _exercise_4_understanding(tp: u8) -> !{
    match tp {
        0 => { _never_return_0()}
        1 => { _never_return_1()}
        2 => { _never_return_2()}
        3 => { _never_return_3()}
        _ => { never_return_fn()}
    };
}

fn _never_return_0() -> ! {
    unimplemented!()
}

fn _never_return_1() -> ! {
    todo!()
}

fn _never_return_2() -> ! {
    panic!();
}

fn _never_return_3() -> ! {
    loop{
        std::thread::sleep(std::time::Duration::from_secs(1))
    }
}

fn _exercise_3(){
    _never_return();
}

use std::thread;
use std::time;

fn _never_return() -> ! {
    loop {
        println!("I return nothing");
        thread::sleep(time::Duration::from_secs(1))
    }
}