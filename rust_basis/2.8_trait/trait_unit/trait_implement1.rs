/*
添加多重约束，实现 Display 和 PartialOrd 的特征
*/
use std::fmt::Display;

struct Pair<T>{
    x: T,
    y: T,
}

// 普通实现 T 的特征
impl<T> Pair<T>{
    fn new(x: T, y: T) -> Self{
        Self{
            x: x,
            y: y,
        }
    }
}

// 实现 T 的 Display 和 PartialOrd 比较特征
impl<T: Display + PartialOrd> Pair<T>{
    fn cmp_display(&self){
        if self.x >= self.y {
            println!("The largest number is x = {}", self.x);
        }else{
            println!("The largest number is y = {}", self.y);
        }
    }
}

fn pair_implement(){
    let pair_example = Pair::new(10i32, 20i32);
    pair_example.cmp_display();
}

/*
有条件地实现特征，为具备某特性的类型实现部分特征
impl<T: Display>  ToString for T{}
*/
fn partial_display(){
    let integer_string = 3i32.to_string();
    let float_string = 12.45f64.to_string();

    println!("partial_display: integer_string {}", integer_string);
    println!("partial_display: float_string {}", float_string);
}

/*
特征作为函数的返回值，以及特征对象
*/
pub struct Weibo{
    username: String,
    content: String,
}

pub struct Post{
    title: String,
    author: String,
    content: String,
}

// 第一步，定义特征，特征中可以只定义函数签名，也可以给出（后续可重载）的默认函数
pub trait Summary{
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Once read more from) {}", self.summarize_author())
    }
}

impl Summary for Weibo{
    fn summarize_author(&self) -> String{
        format!("summarize_author: @{}", self.username)
    }

    fn summarize(&self) -> String{
        format!("Weibo: \"{}\" \twrite an content named\t \"{}\"", self.summarize_author(), self.content)
    }
}

impl Summary for Post{
    fn summarize_author(&self) -> String {
        format!("summarize_author: @{}", self.author)
    }

    fn summarize(&self) -> String{
        format!("Post: \"{}\" \twrite an titled\t \"{}\", which content is\t \"{}\"", self.summarize_author(), self.title, self.content)
    }
}

// Summary 相关特征的实现
fn weibo_and_post_related_trait(){
    let weibo = Weibo{username: String::from("fi-dev-tang"), content: String::from("Penguis can fly to ground")};
    let post = Post{title: String::from("Penguin King"), author: "fi-dev-tang".to_string(), content: "introduce penguin".to_string()};

    println!("weibo info: {}", weibo.summarize());
    println!("post info: {}", post.summarize());
}

/*
目前留一个 bug 在这里，
如果直接写成 impl Summary 的返回值，函数要求只能返回同一类型
之后可以使用特征对象来完善这个 if swith{ Weibo{}}else{ Post{}} 的句式

fn return_summarize(switch: bool) -> Box<dyn Summary>{
    if switch{
        Box::new(Weibo{})
    }else{
        Box::new(Post{})
    }
}
*/
fn return_summarize(switch: bool) -> Box<dyn Summary>{
    if switch{
        Box::new(Weibo{
            username: "fi-dev-tang".to_string(),
            content: "related content about penguin".to_string(),
        })
    }else{
        Box::new(Post{
            title: "penguin chapter".to_string(),
            author: "fi-dev-tang".to_string(),
            content: "posting penguin".to_string(),
        })
    }
}

fn trait_as_return_value(){
    let item1 = return_summarize(false);
    let item2 = return_summarize(true);

    println!("\nitem1 : {}", item1.summarize());
    println!("item2 : {}", item2.summarize());
}

fn main(){
    pair_implement();                       // 限定 Display + PartialOrd 特征实现
    partial_display();                      // 部分实现 impl<T: Display> ToString for T{} 

    weibo_and_post_related_trait();         // 显示 Summary 功能的部分

    trait_as_return_value();                // trait 作为返回值
}