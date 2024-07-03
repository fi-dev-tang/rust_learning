/*
Arrays and slices.
1. An array is a collection of objects of the same type T, stored in contiguous memory.
Arrays are created using brackets [], and their length, which is known at compile time, is part of their type signature
[T; length]

2. Slices are similar to arrays, but their length is not known at compile time.
Instead, a slice is a two-word object.
the first word is a pointer to the data, the second word is the length of the slice.
The word size is the same as usize, determined by the processor architecture, 64 bits on an x86-64.
Slices can be used to borrow a section of an array and have the type signature &[T]
*/
use std::mem;

// This function borrows a slice.
fn analyze_slice(slice: &[i32]){
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

fn main(){
    let xs : [i32; 5] = [1,2,3,4,5];
    
    let ys: [i32; 500] = [0; 500];

    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);

    println!("Number of elements in array:{}", xs.len());

    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    println!("Borrow a section of the array as a slice.");
    analyze_slice(&ys[1..4]);

    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // Same but more verbose

    /*
    arrays 可以使用 .get() 方法，返回的对象是一个 Option, 需要对 Option 进行配对(Some 类型或者 None 类型)
    used with .expect()
    */
    for i in 0..xs.len() + 1{
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }

}