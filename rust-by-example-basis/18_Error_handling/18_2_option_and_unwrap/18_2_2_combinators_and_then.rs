/*
[Combinators]: and_then
直观的思想: 如果 map 调用的 return body 里函数返回类型是 Option<Food>, 则 map 的结果是 Option<Option<Food>> 
and_then 能够做到 Option<Food> 的平坦映射

1. using map() on a function that returns an Option<T> results in the nested Option<Option<T>>.
2. and_then() calls its function input with the wrapped value and returns the result.
If the Option is None, then it returns None instead.
*/
#![allow(dead_code)]

#[derive(Debug)] enum Food{ CordonBleu, Steak, Sushi}
#[derive(Debug)] enum Day {Monday, Tuesday, Wednesday}

// We don't have the ingredients to make Sushi
fn have_ingredients(food: Food) -> Option<Food>{
    match food {
        Food::Sushi => None,
        _ => Some(food),
    }
}

// We have the recipe for everything except Cordon Bleu.
fn have_recipe(food: Food) -> Option<Food>{
    match food {
        Food::CordonBleu => None,
        _ => Some(food),
    }
}

// To make a dish, we need both the ingredients and the recipe.
// We can represent the Logic with a chain of `match`es:
fn cookable_v1(food: Food) -> Option<Food>{
    match have_ingredients(food) {
        None => None,
        Some(food) => match have_recipe(food) {
            None => None,
            Some(food) => Some(food),
        },
    }
}

// This can conveniently be rewritten more compactly with `and_then()`:
/*
解释这里的用法:
have_ingredients(food) 返回一个 Option<Food>
如果 have_ingredients(food) 返回 Some(food), 则 and_then 会将 food 传递给 have_recipe 函数
如果 have_ingredients(food) 返回 None, 则 and_then 直接返回 None
*/
fn cookable_v2(food: Food) -> Option<Food>{
    have_ingredients(food).and_then(have_recipe)
}

fn cookable_v3(food: Food) -> Option<Food>{
    have_ingredients(food).and_then(|food| 
        {
            have_recipe(food)
        }
    )
}

fn eat(food: Food, day: Day){
    match cookable_v3(food) {
        Some(food) => println!("Yay! On {:?} we get to eat {:?}.", day, food),
        None => println!("Oh no.We don't get to eat on {:?}?", day),
    }
}

fn main(){
    let (cordon_bleu, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);

    eat(cordon_bleu, Day::Monday);
    eat(steak, Day::Tuesday);
    eat(sushi, Day::Wednesday);
}