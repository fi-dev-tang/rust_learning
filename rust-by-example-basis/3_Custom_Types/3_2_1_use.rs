/*
The use declaration can be used so manual scoping isn't needed
使用 use 关键字之后，无需手动声明作用域
*/
#![allow(dead_code)]

enum Stage{
    Beginner,
    Advanced,
}

enum Role{
    Student,
    Teacher,
}

fn main(){
    // Explicitly, 
    use crate::Stage::{Beginner, Advanced};
    // Automatically `use` each name inside `Role.`
    use crate::Role::*;

    let stage = Beginner;
    let role = Student;

    match stage {
        // Note the lack of scoping because of the explicit `use` above
        Beginner => println!("Beginners are starting their learning journey!"),
        Advanced => println!("Advanced learners are mastering their subjects..."),
    }

    match role {
        // Note again the lack of scoping.
        Student => println!("Students are acquiring knowledge!"),
        Teacher => println!("Teachers are spreading knowledge!"),
    }
}