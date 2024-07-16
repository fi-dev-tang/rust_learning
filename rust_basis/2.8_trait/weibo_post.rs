/*
对应 2.8.2 Trait 一节, 为不同的文章类型实现 Summary 的特征。特征内只定义函数签名，在每个结构体中单独实现函数
*/
pub trait Summary{
    fn summerize(&self) -> String;
}

pub struct Weibo{
    pub username: String,
    pub content: String,
}

pub struct Post{
    pub author: String,
    pub title: String,
    pub content: String,
}

impl Summary for Weibo{
    fn summerize(&self) -> String{
        format!("The username {} posted {}", self.username, self.content)
    }
}

impl Summary for Post{
    fn summerize(&self) -> String{
        format!("The author {}, which title is {}, content like {}", self.author, self.title, self.content)
    }
}

fn main(){
    let weibo = Weibo{username: "Fi-dev-tang".to_string(), content: "Rust course trait".to_string()};
    let post = Post{author: "Fi-dev-tang".to_string(), title: "Rust-course 2.8.2 trait".to_string(), content: "Rust well done".to_string()};

    println!("{}", weibo.summerize());
    println!("{}", post.summerize());
}