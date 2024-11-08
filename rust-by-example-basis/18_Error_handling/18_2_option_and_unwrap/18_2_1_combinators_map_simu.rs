/*
使用 map 方法，避免 Option 的解包操作，链式方法
定义厨房烹饪过程: peel -> chop -> cook
先定义 tedious(乏味)的多次 match 操作，再定义连续的 map 链式耦合
*/
#![allow(dead_code)]
#[derive(Debug)] enum Food{Apple, Carrot, Potato}
#[derive(Debug)] struct Peeled(Food);
#[derive(Debug)] struct Chopped(Food);
#[derive(Debug)] struct Cooked(Food);

fn peel(food: Option<Food>) -> Option<Peeled>{
    match food {
        Some(food) => Some(Peeled(food)),
        None => None,
    }
}

fn chop(food: Option<Peeled>) -> Option<Chopped>{
    match food{
        Some(Peeled(food)) => Some(Chopped(food)),
        None => None,
    }
}

fn cook(food: Option<Chopped>) -> Option<Cooked>{
    match food{
        Some(Chopped(food)) => Some(Cooked(food)),    // 类似的写法
        None => None,                                 // food.map(|Chopped(food)| Cooked(food))
    }
}

fn eat(food: Option<Cooked>){
    match food {
        Some(Cooked(food)) => println!("We have plenty of {:?} to eat!", food),
        None => println!("This is not eatible"),
    }
}

fn process(food: Option<Food>) -> Option<Cooked>{
    food.map(|f| Peeled(f))
        .map(|Peeled(f)| Chopped(f))
        .map(|Chopped(f)| Cooked(f))
}

fn main(){
    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = None;    

    eat(cook(chop(peel(apple))));
    eat(cook(chop(peel(carrot))));
    eat(cook(chop(peel(potato))));

    let apple_2 = Some(Food::Apple);
    let carrot_2 = Some(Food::Carrot);
    let potato_2 = None;

    eat(process(apple_2));
    eat(process(carrot_2));
    eat(process(potato_2));
}