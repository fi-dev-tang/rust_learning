/*
借用规则的保证前提:
当一个变量的引用存在且有效时，这个变量不能被销毁
这里程序运行和期待的不一致，
原本是第 23 行创建了一个 ref, 期待的现象是 eat_box_i32 不能销毁，因为需要保证引用有效 => 实际编译运行正常
*/
fn eat_box_i32(boxed_i32: Box<i32>){
    println!("Destroying box that contains {}", boxed_i32);
}

fn borrow_i32(borrow_i32: &i32){
    println!("This int is: {}", borrow_i32);
}

fn main(){
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        let _ref_to_i32: &i32 = &boxed_i32;
        eat_box_i32(boxed_i32);
    }
}