fn variable_destruction(){
    let (a, mut b) : (bool, bool) = (true, false);
    // a = true 不可变， b = false 可变
    println!("a = {:?}, b = {:?}", a, b);

    b = true;
    assert_eq!(a,b);
}

fn name_variable(){
    let mut x = 5;
    println!("variable x = {}",x);
    x = 6; 
    println!("variable x = {}",x);

    let _x = 5;
    let _y = 10;
}

/*
解构式赋值: 可以在赋值语句的左式中使用元组、切片和结构体模式。
*/
struct Struct {
    e: i32
}

fn destruction_assign(){
    let (a,b,c,d,e);

    (a,b) = (1,2);
    // _ 代表匹配一个值，但是不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    [c, .., d, _] = [1,2,3,4,5];
    Struct {e , ..} = Struct {e: 5};

    assert_eq!([1,2,1,4,5],[a,b,c,d,e]);
}

/*
Rust 允许声明相同的变量名，在后面的变量名会遮蔽掉前面声明的
变量遮蔽相较于 mut 类型的区别, mut 类型前后的类型要求一致。
*/
fn variable_shadowing(){
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is : {}", x);
    }

    println!("The value of x is: {}", x);

    // 统计一个空格字符串的空格数量
    // 字符串类型
    let spaces = "   ";
    // usize 数值类型
    let spaces = spaces.len();
    println!("spaces = {}",spaces);
    // let mut spaces = "   ";
    // spaces = spaces.len(); 
}

const _MAX_POINTS: u32 = 100_000;

fn main(){
    name_variable();
    variable_destruction();

    destruction_assign();
    variable_shadowing();
}