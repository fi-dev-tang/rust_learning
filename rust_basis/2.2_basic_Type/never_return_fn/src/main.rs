/*
理解成这个程序是守护进程
按正常情况打印执行正确的日志
*/
use std::thread;
use std::time;

fn main(){
    output_log();
}

fn output_log() -> !{
    loop {
        println!("[log] normal execution has passed one second!");
        thread::sleep(time::Duration::from_secs(1));
    }
}