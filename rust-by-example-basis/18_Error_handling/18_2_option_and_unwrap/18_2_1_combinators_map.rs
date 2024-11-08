/*
获得更大的灵活性:
match 是处理 Options 的有效方法。大量使用 match 变得繁琐，特别是只在输入值有效时的操作。
可以使用组合子(combinators) 以模块化的方式管理控制流

Option有一个内置方法叫做 map(), 用于简单映射 Some -> Some 和 None -> None 的组合子。
多个 map() 调用可以串联起来，获得更大的灵活性
*/
#![allow(dead_code)]

#[derive(Debug)] enum Food {Apple, Carrot, Potato}

#[derive(Debug)] struct Peeled(Food);
#[derive(Debug)] struct Chopped(Food);
#[derive(Debug)] struct Cooked(Food);

// Peeling food. If there isn't any, then return `None`.
// otherwise, return the peeled food.
fn peel(food: Option<Food>) -> Option<Peeled>{
    match food {
        Some(food) => Some(Peeled(food)),
        None => None,
    }
}

// Chopping food. If there isn't any, then return `None`.
// otherwise, return the chopped food.
fn chop(peeled: Option<Peeled>) -> Option<Chopped>{
    match peeled {
        Some(Peeled(food)) => Some(Chopped(food)),
        None => None,
    }
}

// Cooking food. Here, we showcase `map()` instead of `match` for case handling.
fn cook(chopped: Option<Chopped>) -> Option<Cooked>{
    chopped.map(|Chopped(food)| Cooked(food))
}

// A function to peel, chop and cook food all in sequence.
// We chain multiple uses of `map()` to simplify the code.
fn process(food: Option<Food>) -> Option<Cooked> {
    food.map(|f| Peeled(f))
        .map(|Peeled(f)| Chopped(f))
        .map(|Chopped(f)| Cooked(f))
}

// Check whether there's food or not before trying to eat it!
fn eat(food: Option<Cooked>){
    match food{
        Some(food) => println!("Mmm. I love {:?}", food),
        None => println!("Oh no! It wasn't edible."),
    }
}


fn main(){
    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = None;

    let cooked_apple = cook(chop(peel(apple)));
    let cooked_carrot = cook(chop(peel(carrot)));
    let cooked_potato = process(potato);

    eat(cooked_apple);
    eat(cooked_carrot);
    eat(cooked_potato);
}