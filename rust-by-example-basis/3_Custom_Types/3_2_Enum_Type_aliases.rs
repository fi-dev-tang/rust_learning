/*
Type aliases:
If you use type alias, you can refer to each enum variant via its alias.
This might be useful if the enum's name is too long or too generic, and you want to rename it.
*/
enum VeryVerboseEnumOfThingsToDoWithNumber{
    Add,
    Subtract,
}

// Create a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumber;

/*
The common place you'll see this is in impl blocks using the Self alias.
*/
impl VeryVerboseEnumOfThingsToDoWithNumber {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self{
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

fn main(){
    let x = Operations::Add;
    let y = Operations::Subtract;

    /*
    关于一则报错的 check:
    如果声明 let x = Operation::Add;
    并且调用过程中使用 println!("{}", run(&x, 1, 1));
    产生报错: run(&x, 1, 1) not found in this scope 
    报错建议: use the `.` operator to call the method `run` on `&VeryVerboseEnumOfThingsToDoWithNumber`
    */
    println!("1 + 1 = {}", x.run(1,1));
    println!("1 - 1 = {}", y.run(1,1));
}