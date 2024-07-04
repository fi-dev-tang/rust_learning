/* 使用 use , 减少后续重复作用域使用 */
#![allow(dead_code)]
enum Stage{
    Beginner,
    Advanced,
}

enum Record{
    Student,
    Teacher,
}

fn main(){
    use crate::Stage::{Beginner, Advanced};
    use crate::Record::*;

    let beginner = Beginner;
    let student = Student;

    match beginner {
        Beginner => println!("this is beginner!"),
        Advanced => println!("this is advanced!"),
    }

    match student {
        Student => println!("this is a student"),
        Teacher => println!("this is a teacher"),
    }
}