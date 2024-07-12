/* 为泛型实现 copy 或 Clone trait 的对比 */
fn largest_copy<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T{
    let mut largest = list[0];

    for &iter in list.iter() {
        if iter > largest{
            largest = iter
        }
    }
    largest
}

fn largest_clone<T: std::cmp::PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();

    for iter in list.iter() {
        if *iter > largest{
            largest = (*iter).clone()
        }
    }
    largest
}

fn main(){
    let number_list = [1, 3, 17, 9, 22, 4];
    let char_list = ['a', 'c', 'q', 'w'];
    
    println!("copy number: {}", largest_copy(&number_list));
    println!("copy character: {}", largest_copy(&char_list));

    println!("clone number: {}", largest_clone(&number_list));
    println!("clone character: {}", largest_clone(&char_list));
}