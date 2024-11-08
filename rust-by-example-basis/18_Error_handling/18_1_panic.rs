/*
painc 的使用场景: 分解任务(unwinding the task),
显示地抛出错误情形
*/
fn give_princess(gift: &str){
    // Princesses hate snakes, so we need to stop if she disapproves!
    if gift == "snake" {
        panic!("AAAAAaaaaaa!!");
    }

    println!("I love {}s!!!!!", gift);
}

fn main(){
    give_princess("teddy bear");
    give_princess("snake");
}