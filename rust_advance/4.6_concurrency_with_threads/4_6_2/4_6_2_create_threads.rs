/* 创建线程 */
use std::thread;
use std::time::Duration;

fn main(){
    thread::spawn(|| {
        for i in 1..10 {
            println!("number {} from spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    /*
    线程内部的代码使用闭包来执行
    */

    for i in 1..5 {
        println!("number {} from main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}