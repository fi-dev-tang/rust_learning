pub struct Weibo{
    pub author: String,
    pub title: String,
    pub content: String,
}

pub struct Post{
    pub username: String,
    pub content: String,
}

pub trait Summary{
    fn summarize_author(&self) -> String;   // 仅使用函数签名，无默认实现
    fn summarize(&self) -> String{
        format!("(Read more)...{}", self.summarize_author())
    }
}

impl Summary for Post{
    fn summarize_author(&self) -> String{
        format!("post@_ {}", self.username)
    }
}

impl Summary for Weibo{
    fn summarize_author(&self) -> String{
        format!("weibo@_ {}", self.author)
    }
}

// 使用特征作为函数参数
pub fn notify(item: &impl Summary){
    println!("Breaking news {}", item.summarize());
}

fn main(){
    let weibo = Weibo{author: "fi-dev-tang".to_string(), title: "article title".to_string(), content: "rust course".to_string()};
    let post = Post{username: "fi-dev-tang".to_string(), content: "book title".to_string()};

    println!("{}", weibo.summarize());
    println!("{}", post.summarize());
    notify(&weibo);
    notify(&post);
}