/*
使用函数式编程简化代码行数的一个简单的例子:
原因: 函数可以传递给变量

1. 这里确实做到了最初步的修改，如果调用的动作从 muuuu 变成了 woooo, 原本的程序需要修改几十处 muuuu 出现的地方，
但这里使用 action = muuuu, 只需要修改这一处，其他地方不需要修改。
2. 尝试把 action 修改成函数闭包，讨论区别:
目前为止闭包的好处就是可以捕获作用域中的变量，但认为这比较显然

尝试再次总结一下闭包的使用: 实际上在 python 中不难理解
def outer_function(x):
    def inner_function(y):
        return x + y
    return inner_function

我认为还是比较自然的，这里内部函数是否能访问作用域之外的变量，是由语言运行时(runtime)决定的，

【psNote: 闭包和普通函数的区别】
某些编程语言中，普通函数只能访问全局作用域中的变量和其自身局部作用域中的变量，不会自动捕获定义它们的外部作用域中的变量。
闭包可以捕获或记住其创建时的作用域中的变量，并在以后访问它们。
这样一来，即使外部函数已经执行完毕，闭包依然可以访问那些捕获的变量。

强调的是捕获其创建时的环境。
*/
use std::thread;
use std::time::Duration;

fn muuuu(intensity: u32) -> u32{
    println!("muuuuuuuuuuuu.........");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn workout(intensity: u32, random_number: u32){
    let action = muuuu;
    if intensity < 25 {
        println!("Today, do {} pushups!", action(intensity));
        println!("Next, do {} situps!", action(intensity));
    }
    else if random_number == 3{
        println!("Take a break today!");
    }else{
        println!("Today, run for {} minutes!", action(intensity));
    }
}

fn main(){
    let intensity = 10;
    let random_number = 7;
    workout(intensity, random_number);
}