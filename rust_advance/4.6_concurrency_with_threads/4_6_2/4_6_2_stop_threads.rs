/*
[线程是如何结束的?]
main 线程是主线程，一旦结束，程序就随之结束。各个子线程也会被强行终止。
父线程不是主线程 --> Rust 会等待线程的代码执行完，线程自动结束

讨论情况: 线程的代码不会执行完?
1. 线程任务是 IO 循环读取
    IO阻塞，等待读取新的数据 -> 读到数据，处理完成 -> 继续阻塞等待 -> 收到 socket 关闭的信号 -> 结束线程
绝大部分时间处于阻塞状态，CPU占用很小，网络服务中最常见的模型

2. 线程任务是一个循环
里面没有任何阻塞，也没有休眠操作，CPU会被跑满
*/
use std::thread;
use std::time::Duration;

fn main(){
    // 创建线程 A
    // 线程 A 的任务是创建线程 B, 创建完成后，属于线程 A 的执行流即结束
    let a_thread = thread::spawn(|| {
        thread::spawn(|| {
            loop{
                println!("I am thread B, created by thread A");
            }
        });
    });

    a_thread.join().unwrap();
    println!("A thread finished!");

    thread::sleep(Duration::from_millis(1));
}