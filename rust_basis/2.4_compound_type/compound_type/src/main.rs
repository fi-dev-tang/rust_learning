/*
复合类型举例:
如果直接使用 type File = String,
调用 open(&mut f1); close(&mut f1); 远没有 f1.open() 或 f1.close() 调用直观
*/
#![allow(unused_variables)]
type File = String; // 定义 File 类型为 String


fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

#[allow(dead_code)]
fn read(f: &mut File, _save_to: &mut Vec<u8>) -> !{
    unimplemented!()
}

fn file_operation(){
    let mut f = File::from("f1.txt");
    open(&mut f);
    // read(&mut f, &mut vec![]);
    close(&mut f);
}

/*
字符串
*/
// 区分 String 和 &str 类型
fn generate_string(){
    let a_string = "hello";
    greet_string(a_string.to_string());
}

fn greet_string(a_string: String){
    println!("{}, world", a_string);
}

// 字符串切片
fn string_slice_example(){
    let a_string = String::from("try to use a string");

    let a_half = &a_string[0 .. a_string.len()/2];
    let b_half = &a_string[a_string.len()/2 .. a_string.len()];

    let total_string = &a_string[0..];

    println!("{} - {}", a_half, b_half);
    println!("{}", total_string);
}

// 字符串切片中文编码: UTF-8, 一个中文字符占 3 个字节
fn string_utf8(){
    let s = "中国人";
    let output_string = &s[0..3];
    // Error: byte index 2 is not a char boundary; it is inside '中'(bytes 0..3) of `中国人`
    println!("{}", output_string);
}

// 根据字符串，返回其切片
fn return_slice_from_string(){
    let s = String::from("a string");
    let first_word = return_slice(&s);
    // s.clear(); 
    // 错误! .clear() 的作用是暂时清空 String 变量 s 的内容，使其长度变为 0,容量保持不变
    /*
    s.clear() 的说明:
    pub fn clear(&mut self), 参数是对自身的可变借用，之后 println! 又使用了不可变借用，
    在 s.clear() 处可变借用和不可变借用试图同时生效，因此编译无法通过。
    */
    println!("first_word: {}", first_word);
}

fn return_slice(a_string: &String) -> &str{
    &a_string[..1]
}

fn other_slice(){  // 数组切片 &[i32], 其类型为 &[i32],工作方式与字符串切片相同，持有一个引用指向原始数组的某个元素和长度。
    let a = [1,2,3,4,5];
    let slice_a = &a[1..3];

    println!("{:?}", slice_a);
    assert_eq!(slice_a, &[2,3]);
}

/*
1. 从 &str 转为 String 的两种方式
String::from("hello, world");
let s = "hello, world"; s.to_string()
2. 从 String 转为 &str 的方式（实际上取引用即可)
*/
fn convert_string_to_str(){
    let s_string = String::from("ahahaha");
    say_hello(&s_string);
    say_hello(&s_string[..]);
    say_hello(s_string.as_str());

}

fn say_hello(a_str: &str){
    println!("say_hello: {}", a_str);
}

/*
fn test_string_index(){
    let s = String::from("test_string_from_index");
    let h = s[0];
}
// 使用索引会导致报错
*/

// 字符串追加 Push
fn push_string_example(){
    let mut s = String::from("Hello,");
    s.push_str(" Rust");

    println!("push_str: {}", s);
    
    s.push('!');
    println!("push: {}", s);
}

// 字符串插入 Insert
fn insert_string_example(){
    let mut s = String::from("Hello Rust!");
    s.insert(5, '?');

    println!("insert: {}", s);

    s.insert_str(7, "Welcome the world of ");
    println!("insert_string: {}", s);
}

// 字符串替换
/*
replace, 返回一个新字符串，而不是替换原有的字符串
*/
fn string_replace_method_1(){
    let a_string = String::from("I start to learn rust, rust is fun!");
    let new_string = a_string.replace("rust", "Rust");

    println!("a_string:{}   new_string:{}", a_string, new_string);
    dbg!(new_string);
}

/*
replacen, 需要标明替换的个数
返回一个新字符串，而不是替换原有的字符串
*/
fn string_replace_method_2(){
    let a_string = String::from("I love rust, rust is great!");
    let new_string = a_string.replacen("rust", "Rust", 1);

    dbg!(new_string);
}

/*
replace_range
需要使用 mut 关键字，在原有的字符串上进行操作
*/
fn string_replace_method_3(){
    let mut a_string = String::from("I like rust");
    a_string.replace_range(7..8, "R");
    dbg!(a_string);
}

/*
rust 删除操作 Pop(), 删除并返回字符串的最后一个字符
直接操作原来的字符串，但存在返回值
*/
fn string_delete_rust_pop(){
    let mut a_string = String::from("I like Rust 说中文!");
    
    let p1 = a_string.pop();
    let p2 = a_string.pop();

    dbg!(p1);
    dbg!(p2);
    dbg!(a_string);
}

/*
remove: 直接操作原来的字符串。
删除并返回指定位置的字符
*/
fn string_delete_rust_remove(){
    let mut remove_s = String::from("测试remove方法");
    println!("remove_s 占 {} 个字节", std::mem::size_of_val(remove_s.as_str()));

    remove_s.remove(3);
    dbg!(remove_s);
}

/*
直接操作原字符串，删除从指定位置到结尾的字符
truncate()
*/
fn string_delete_rust_truncate(){
    let mut truncate_s = String::from("测试remove方法");
    println!("{} 所占的字节数是 {}", truncate_s, std::mem::size_of_val(truncate_s.as_str()));

    truncate_s.truncate(3);
    dbg!(truncate_s);
}

/*
清空字符串, 相当于 truncate(0)
*/
fn string_delete_rust_clear(){
    let mut clear_s = String::from("测试clear方法");
    clear_s.clear();
    dbg!(clear_s);
}

/*
连接
1. 使用 + 或者 += 连接字符串
使用 + 或者 += 连接字符串，要求右边的参数必须为字符串的切片引用 (Slice) 类型。
当调用 + 操作符的时候，相当于调用了 std::string 标准库中的 add() 方法。
这里 add() 方法的第二个参数是一个引用的类型。
当我们在使用 + 时，必须传递切片引用类型，不能直接传递 String 类型，+ 是返回一个新的字符串，可以不需要 mut 关键字
*/
fn string_concatenate_1(){
    let a_string = String::from("hello");
    let b_string = String::from(", Rust");

    let mut result = a_string + &b_string;
    result += "!!!";

    dbg!(result);
}

/*
连接操作说明:
fn add(self, s: &str) -> String
其中前者的所有权会被转移
[add 解释]:
s1 这个变量通过调用 add() 方法后，所有权被转移到 add() 方法里面，
add() 方法调用后就被释放了，同时 s1 也被释放了。再使用 s1 就会发生错误。这里涉及到所有权转移(Move)的相关知识。
*/
fn add_example(){
    let s1 = String::from("Hello");
    let s2 = String::from(",Rust");
    /*
    在下句中，s1 的所有权被转移走了，因此后面不能再使用 s1
    */
    let s3 = s1 + &s2;

    assert_eq!(s3, "Hello,Rust");
    // println!("{}", s1);  // 报错
}

/*
连接操作的第二种使用, format! 类似于 print!
*/
fn string_concatenate_2(){
    let s1 = String::from("Hello");
    let s2 = String::from("Rust");

    let s = format!("{},{}", s1, s2);
    println!("s = {}", s);
}


fn main(){
    file_operation();           // 用复合类型实现文件的打开，读取，关闭操作 API 模拟（没有真实的实现)

    generate_string();          // 直接用 let a_string = "hello", 不能转成 String 类型

    string_slice_example();     // 字符串切片格式 let s = String::from("hello world"); let half = &s[0..len()] 格式

    string_utf8();              // 字符串中文(UTF-8), 一个中文字符占据 3 个字节

    return_slice_from_string(); // 根据字符串，返回其切片

    other_slice();              // 数组切片

    convert_string_to_str();    // String 转换为 &str

    // test_string_index();
    push_string_example();      // 字符串追加

    insert_string_example();    // 字符串插入

    string_replace_method_1();  // 字符串替换方法 1

    string_replace_method_2();

    string_replace_method_3();

    // 删除操作
    string_delete_rust_pop();
    string_delete_rust_remove();
    string_delete_rust_truncate();
    string_delete_rust_clear();

    // 连接操作
    string_concatenate_1();

    // 连接操作的 add 说明
    add_example();

    string_concatenate_2();
}