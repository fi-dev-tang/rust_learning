/*
使用闭包去捕获外界的 indensity
*/
use std::thread;
use std::time::Duration;

fn workout(indensity: u32, random_number: u32) -> impl Fn() -> u32{
    let action = move || {
        println!("muuuuu.......");
        thread::sleep(Duration::from_secs(2));
        indensity
    };

    if indensity < 25{
        println!("do {} pushups", action());
        println!("do {} situps", action());
    }else if random_number == 3{
        println!("take a break today! Remember to stay hydrated!");
    }else{
        println!("run for {} minutes", action());
    }
    return action
}


fn main(){
    let indensity = 10;
    let random_number = 7;

    println!("{}", workout(indensity, random_number)());
}