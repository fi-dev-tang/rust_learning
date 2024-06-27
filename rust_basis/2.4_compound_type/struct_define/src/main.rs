/*
定义结构体和实例化结构体
*/
#[derive(Debug)]
struct User{
    active: bool,
    email:String,
    user_name: String,
    sign_in_count: u64,
}

fn examplize_struct(){
    let mut user1 = User{
        email: String::from("user1@qq.com"),
        user_name: String::from("user1"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("use1_change@qq.com");
    println!("{:?}", user1);
}

/*
简化结构体构造的写法
类似其它语言的构造函数
*/
fn simplify_user_build_method1(email: String, user_name: String) -> User{
    User{
        email: email,
        user_name: user_name,
        active: true,
        sign_in_count: 1,
    }
}

/*
简化结构体构造的写法 2，
对于结构体字段和函数参数同名，直接将 email: email 省略成 email
*/
fn build_user(email:String, user_name: String) -> User {
    User{
        email,
        user_name,
        active: true,
        sign_in_count: 1,
    }
}

fn build_user_test(){
    let user1_email = String::from("user1_method1@google.com");
    let user1_name = String::from("user1_name");

    let user1 = simplify_user_build_method1(user1_email, user1_name);

    let user2_email = String::from("user2_method2@google.com");
    let user2_name = String::from("user2_name");

    let user2 = build_user(user2_email, user2_name);

    println!("{:?}", user1);
    println!("{:?}", user2);
}

/*
从已有的结构体对象实例化新的对象
*/
fn examplize_from_old(){
    let user1 = build_user("zhangsan@qq.com".to_string(), "zhangsan".to_string());
    let user2 = User {
        email: String::from("lisi@qq.com"),
        user_name: String::from("lisi"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    let user3 = User {
        email: String::from("wangwu@qq.com"),
        ..user1 
    };  // 这里是完全的所有权转移，会显示: value moved here after borrow

    println!("{:?}", user2);
    println!("{:?}", user3);
}

/*
struct 在内存中的排列
采用 #[derive(Debug)] 显示 println!("{:}?"); 打印相关的 debug 信息和细节
*/
#[derive(Debug)]
struct File{
    name: String,
    data: Vec<u8>,
}

fn struct_in_memorylayout(){
    let f1 = File{
        name: String::from("f1.txt"),
        data: Vec::new(),
    };

    let f1_name = &f1.name;
    let f1_length = &f1.data.len();

    println!("f1: {:#?}", f1);
    println!("f1's name {} and f1's length {}", f1_name, f1_length);
}

/*
定义元组结构体，关心结构体的名称，但不关心结构体内部字段的名称
*/
#[derive(Debug)]
struct Point(i32, i32, i32);
#[derive(Debug)]
struct Color(i32, i32, i32);

fn tuple_struct(){
    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    println!("{:?}", black);
    println!("{:?}", origin);
}

/*
单元结构体(Unit-like struct)
struct AlwaysEqual;
let subject = AlwaysEqual;
impl someTrait for AlwaysEqual{}
*/

/*
避免在结构体中使用引用：因为引用的对象不具有所有权，需要引入生命周期的概念，来限制 结构体成员的所有权范围 小于借用数据的所有权范围
生命周期确保: 结构体的作用范围要比 它所借用的数据的作用范围要小。
*/
#[derive(Debug)]
struct UserLife{    // expected named lifetime parameter
    username: &'static str,
    email: &'static str,
}

// 避免使用:
fn struct_lifetime_missing(){
    let userlife = UserLife{
        username: "zhangsan",
        email: "zhangsan@qq.com"
    };

    println!("{:#?}", userlife);        // 一种更好的输出格式: {:?} 或者 {:#?}
}

/*
使用 dbg! 打印结构体信息
*/
#[derive(Debug)]
struct Rectangle{
    width: i32,
    height: i32,
}

fn using_dbg_macro(){
    let scale = 2;
    let rect = Rectangle{
        width: dbg!(scale * 20),
        height: 30,
    };

    dbg!(&rect);
    println!("{:#?}",rect);
}

fn main(){
    examplize_struct();     // 实例化结构体
    build_user_test();      // 通过类似构造函数简化结构体实例化
    examplize_from_old();   // 通过旧实例创建新实例

    struct_in_memorylayout(); // struct 在内存中的排列
    tuple_struct();           // 元组结构体

    struct_lifetime_missing(); // struct 中定义引用类型 需要添加生命周期
    using_dbg_macro();         // 使用 macro 宏进行打印
}