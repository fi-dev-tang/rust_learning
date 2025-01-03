/*
使用闭包来简化代码，先写一个基础的实现
*/
use std::thread;
use std::time::Duration;

fn muuuuu(intensity: u32) -> u32{
    println!("muuuuuuuuuuuu........");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn workout(intensity: u32, random_number: u32){
    if intensity < 25{
        println!("Today, do {} pushups!", muuuuu(intensity));
        println!("Next, do {} situps!", muuuuu(intensity));
    }else if random_number == 3{
        println!("Take a break today! Remember to stay hydrated!");
    }else{
        println!("Today, run for {} minutes!", muuuuu(intensity));
    }
}

fn main(){
    let intensity = 10;
    let random_number = 7;
    workout(intensity, random_number);
}