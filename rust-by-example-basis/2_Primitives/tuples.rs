/*
tuple is a value with type signature (T1, T2, ...)
where T1, T2 are the types of its members.
FUnctions can use tuples to return multiple values, as tuples can hold any number of values.
*/
fn reverse(pair: (i32, bool)) -> (bool, i32){
    // `let` can be used to bind the members of a tuple to variables.
    let (int_param, bool_param) = pair;
    (bool_param, int_param)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main(){
    // A tuple with a bunch of different types.
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                    -1i8, -2i16, -3i32, -4i64,
                    0.1f32, 0.2f64,
                    'a', true);
    
    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);

    // Tuples can be tuple members.
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let pair = (1, true);
    println!("Pair is {:?}", pair);

    println!("The reversed pair is {:?}", reverse(pair));

    println!("One element tuple: {:?}", (5u32,));
    println!("Just an integer: {:?}", (5u32));

    let tuple = (1, "hello", 4.5, true);

    let (a,b,c,d) = tuple;
    println!("{:?},{:?},{:?},{:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
}