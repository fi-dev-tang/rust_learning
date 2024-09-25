/*
新的线程引用当前环境的变量时，在闭包中使用 move 关键字获取变量的所有权，同时原线程不能再使用
*/
use std::thread;

fn main(){
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("borrowed from main thread {:?}", v);
        drop(v);
    });

    handle.join().unwrap();
}