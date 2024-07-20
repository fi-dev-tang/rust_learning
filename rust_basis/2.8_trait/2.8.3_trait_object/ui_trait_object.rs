/*
利用特征对象实现无限增加的 UI 列表打印
*/
struct Button{
    width: f32,
    height: f32,
    label: String,
}

struct SelectBox{
    width: f64,
    height: f64,
    content: Vec<String>, // 动态数组实现
}

trait Draw{ // 实现 Draw 特征
    fn draw(&self) -> String;
}

impl Draw for Button{
    fn draw(&self) -> String{
        format!("width:{}, height:{}, label:{}", self.width, self.height, self.label)
    }
}

/*
对 selectBox 的写法修改，希望也能打印出 content 中动态字符串的每一项
*/
impl Draw for SelectBox{
    fn draw(&self) -> String{
        // 初始化一个字符串存储所有的内容项
        let mut result = format!("width:{}, height:{}, content:", self.width, self.height);
        for (index, item) in self.content.iter().enumerate(){
            // 为每个内容添加索引和内容，根据需要调整格式
            result.push_str(&format!("[{}:{}] ", index, item));
        }
        result
    }
}

// 实现一个大型的动态数组 Screen
// 其成员是特征对象 Box<dyn Draw>
struct Screen{
    components: Vec<Box<dyn Draw>>,
}

impl Screen{
    fn run(&self){
        for object in self.components.iter(){
            println!("{}", object.draw());
        }
    }
}

fn main(){
    let ui_screen = Screen{
        components: vec![
            Box::new(Button{width: 3.2, height: 2.8, label: String::from("this is a box")}),
            Box::new(SelectBox{width: 2.6, height: 3.1415, content:
                vec![String::from("SelectBox begin"), String::from("content is penguin"), String::from("SelectBox end")],
            }),
            Box::new(SelectBox{width: 30.1, height: 33.22, content:
                vec![String::from("Hello rust"), String::from("Chapter 2.8.3 trait object"), String::from("Rust Course")],
            }),
        ],
    };

    ui_screen.run();
}