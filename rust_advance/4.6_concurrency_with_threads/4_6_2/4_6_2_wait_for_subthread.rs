/* 
等待子线程的结束
需要一个方法，让主线程安全、可靠地等所有子线程完成任务后，再 kill self。
*/
use std::thread;
use std::time::Duration;

fn main(){
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Spawned thread say hi the {}th time!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });   
    /*
    [handle] 的具体说明:
    handle 是 thread::spawn 方法返回的一个 JoinHandle, 它代表了这个新创建的子线程。
    调用 Join 方法时，它会阻塞调用它的线程，直到对应的子线程运行完毕。--> join 理解成: 谁创建我，我阻塞谁
    */

    for i in 1..5 {
        println!("Main thread say hello the {}th time!", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();         // 阻塞主线程，等待执行的子线程完成， 这里移动了 handle.join().unwrap() 的位置，发现交织的打印结果
}