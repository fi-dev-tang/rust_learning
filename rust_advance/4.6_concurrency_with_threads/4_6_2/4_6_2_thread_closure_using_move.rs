/*
线程闭包中使用 move
[move 关键字在闭包中的使用]: 让该闭包拿走环境中某个值的所有权。
move: 将所有权从一个线程转移到另外一个线程
*/
//============================= step 1. 【直接在一个线程中使用另一个线程的数据】 ========================================
/*
use std::thread;

fn main(){
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
*/
/*
[note] 代码本身没有问题，问题在于 Rust 无法确定新的线程会活多久(多个线程的结束顺序并不是固定的)，
所以无法确定新线程所引用的 v 是否在使用过程中一直合法。
*/
//================================== step 2. 展示子线程结束，主线程未结束的问题 ===============================
use std::thread;

fn main(){
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    drop(v);
    handle.join().unwrap();
}