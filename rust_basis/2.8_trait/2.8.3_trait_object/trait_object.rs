#[derive(Debug)]
enum UiObject{
    Button,
    SelectBox,
}

fn draw(o: UiObject){
    println!("{:?}", o);
}

fn draw_uiobject(){
    let ui_list = [
        UiObject::Button,   UiObject::SelectBox,
        UiObject::SelectBox,    UiObject::Button,
    ];

    println!("draw_uiobject:");
    for o in ui_list{
        draw(o);
    }
}

// 下面为实现特征对象的部分
/*
特征对象的使用场景: 类似于面向对象中的类和继承关系
当不确定具体需要创建多少个类时，或者说任意新增一个类型时，使用特征对象
用法: Box<dyn Trait> 或者 &dyn Trait
Box 智能指针本身具有解引用的特征 Deref, 操作 Box<dyn Trait> 类型的对象就相当于操作 T 本身
*/
// 定义特征 draw, 只给出函数签名
trait Draw{
    fn draw(&self) -> String;
}

// 为类型 u32 实现 Draw 的特征
impl Draw for u32{
    fn draw(&self) -> String{
        format!("draw u32 {:?}", self)
    }
}

// 为类型 f64 实现 Draw 的特征
impl Draw for f64{
    fn draw(&self) -> String{
        format!("draw f64 {:?}", self)
    }
}

// 特征对象的使用: 只要满足 impl Draw, 都可以传入作为参数, 传入参数的类型为: Box<dyn Draw>
fn draw1(x: Box<dyn Draw>) -> String{
    x.draw()
}

// 特征对象的使用: 传入参数类型为 &dyn Draw
fn draw2(x: &dyn Draw) -> String{
    x.draw()
}

fn trait_object_implement(){
    let x = 12u32;
    let y = 3.4f64;

    println!("draw1: {}", draw1(Box::new(x))); 
    println!("draw1: {}", draw1(Box::new(y)));
    println!("draw2: {}", draw2(&x));
    println!("draw2: {}", draw2(&y));
}

fn main(){
    draw_uiobject();                // 依次渲染每个列表里不同的对象
    trait_object_implement();       // 特征对象的实现: Box<dyn Trait> 或者 &dyn Trait   
}