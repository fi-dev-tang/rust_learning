/*
演示手动释放 ManuallyDrop 功能的使用，
以及使用自己控制生命周期的构建方法 from_raw_parts() 生成 String
*/
use std::mem;
fn main(){
    let story = String::from("Rust's practice!");
    let mut story = mem::ManuallyDrop::new(story);

    let ptr = story.as_mut_ptr();
    let len = story.len();
    let capacity = story.capacity();

    let s = unsafe{String::from_raw_parts(ptr, len, capacity)};
    assert_eq!(s, *story);
    assert_eq!(len ,16);

    println!("s = {}", s);
    println!("story = {:?}", story);
}