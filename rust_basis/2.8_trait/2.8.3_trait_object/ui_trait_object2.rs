/* 实现 UI 功能渲染，关注 Vec<String> 的字符串拼接，以及 components 作为特征对象的动态数组 */
struct Botton{
    width: f32,
    height: f32,
    label: String,
}

struct SelectBox{
    width: f32,
    height: f32,
    content: Vec<String>,
}

trait Draw{
    fn draw(&self) -> String;
}

impl Draw for Botton{
    fn draw(&self) -> String{
        format!("Botton: width:{}, height:{}, label:{}", self.width, self.height, self.label)
    }
}

impl Draw for SelectBox{
    fn draw(&self) -> String{
        let mut result = format!("SelectBox: width:{}, height:{}, content:", self.width, self.height);
        for (index, item) in self.content.iter().enumerate() {
            result.push_str(&format!("[{}: {}] ", index, item));
        }
        result
    }
}

struct Screen{
    components: Vec<Box<dyn Draw>>,
}

impl Screen{
    fn run(&self){
        for component in self.components.iter() {
            println!("{}", component.draw());
        }
    }
}

fn main(){
    let ui_screen = Screen{
        components: vec![
            Box::new(Botton{width: 12.3, height: 35.6, label:String::from("Output string")}),
            Box::new(SelectBox{width: 26.80, height: 70.50,
            content: vec![String::from("Rust 2.8.3"), String::from("screen output Box"), String::from("Box<dyn Draw>")]}),
            Box::new(SelectBox{width: 79.60, height: 80.50,
            content: vec![String::from("& Trait object"),
            String::from("object oriented"),   String::from("Rust screen selectBox"), String::from("components"),
            ]}),
        ],
    };
    ui_screen.run();
}