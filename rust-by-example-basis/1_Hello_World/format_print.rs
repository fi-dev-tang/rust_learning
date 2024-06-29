fn main(){
    println!("{} days", 21);
    
    // Positional arguments can be used. Specifying an interger inside `{}`, Arguments start at 0 immediately after the format string.
    println!("{0}, this is {1}. {1} this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!("{subject}{verb}{object}",
    object = "the lazy dog",
    subject = "the quick brown fox", 
    verb = "jumps over");

    // different formatting can be invoked by specifying the format character after a `:`
    println!("Base 10:           {}", 69420);    // 69420
    println!("Base 2 (binary):  {:b}", 69420);   
    println!("Base 8 (octal):   {:o}", 69420);
    println!("Base 16(hexadecimal): {:x}", 69420);

    // 可以使用指定的宽度右对齐
    println!("{number:>5}", number=1);

    // you can pad numbers with extra zeroes
    // 格式就是: println!("number:填充的数字 < or > 对齐的位数")， 如果 1 是number，那么 < 表示 1 在左侧，> 表示 1 在右侧
    println!("{number:0>5}",number = 1);
    println!("{number:0<5}", number = 1);

    println!("{number:0>width$}", number = 1, width = 5);

    println!("My name is {0}, {1} {0}", "Bond", "James");

    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    #[derive(Debug)]
    struct Structure(i32);      // 声明了一个叫做 Structure 的结构体，匿名成员，元组结构体

    println!("This struct `{:?}` won't print...", Structure(3));
    // For Rust 1.58, directly capture the argument from a surrounding variable. Just like the above, this will output "    1", 4 white spaces and a "1"
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
    let pi: f64 = 3.141592;
    println!("Pi is roughly {number:.prec$}", number = pi, prec = 3);
}