/*
类型别名
1. type = ...
2. impl 使用 Self 代替
*/
enum VeryLongAndUselessAddAndSubstractionImplementation{
    Add,
    Substract,
}

type Operation = VeryLongAndUselessAddAndSubstractionImplementation;

impl VeryLongAndUselessAddAndSubstractionImplementation{
    fn run(&self, x: i32, y: i32) -> i32 {
        match self{
            Self::Add => x + y,
            Self::Substract => x - y,
        }
    }
}

fn main(){
    let add_op = Operation::Add;
    let substraction_op = Operation::Substract;

    println!("add: {}", add_op.run(1, 1));
    println!("sub: {}", substraction_op.run(1, 1));
}