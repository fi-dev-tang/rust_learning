/*
使用 多线程构建线程安全的哈希表
每个线程都能获取哈希表的索引，由该索引得到每次插入时的锁
*/
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::thread;
use rand::prelude::*;

// 类型定义一个线程安全的哈希表
type SafeHashMap = Arc<Mutex<HashMap<u32, u32>>>;

fn main(){
    // 1. 创建一个线程安全的哈希表
    let ht: SafeHashMap = Arc::new(Mutex::new(HashMap::new()));

    // 2. 设置线程数 和每个线程插入表项的条目数
    let num_of_threads = 10;
    let item_of_insertion = 1000;

    // 2.1 对每个线程执行 join, 设置一个 mutable 的线程表
    let mut handles = vec![];

    for i in 0..num_of_threads {
        // 3.0 获取哈希表的索引
        let ht = Arc::clone(&ht);

        // 3. 开始创建线程
        let handle = thread::spawn(move || {
            for _ in 0..item_of_insertion {
                let key = thread_rng().gen::<u32>();
                let value = thread_rng().gen::<u32>();

                let mut ht_mutex = ht.lock().unwrap();  // 获取哈希表的互斥表
                ht_mutex.insert(key, value);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();     // 等待每个线程执行完成
    }

    let mut ht_mutex = ht.lock().unwrap();
    println!("The item in ht is {}", ht_mutex.len());
}