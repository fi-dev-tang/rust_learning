/*
一个例子说明 map() 和 and_then() 的区别:
1. 和之前 18.2.1 的例子并不矛盾
当时的类型定义是 enum Food; struct Peeled(Food); struct Chopped(Food); 
在 map里定义 .map(|f| Peeled(f)).map(|f| Chopped(f))
return body 里都是 struct 类型
2. 如果定义 .map(|f| function(f))
而这个 function(f) 返回的是 Option<Food> 类型
在 map() 本身的包裹作用下，会出现: Some(Some(food)) 或者 Some(None) 的情况
造成后续模式匹配的时候，还需要多次匹配  Some(Some(food))
因此不如使用 and_then, 保证每次最终只产生一个 Option<> 包装
*/
#[derive(Debug)]
enum Ingredient{
    Meat,
    Vegetable
}

#[derive(Debug)]
enum Food{
    Steak,
    Salad,
}

fn prepare_meal(ingredient: Ingredient) -> Option<Food>{
    match ingredient {
        Ingredient::Meat => Some(Food::Steak),
        Ingredient::Vegetable => Some(Food::Salad),
    }
}

fn check_seasoning(food: Food) -> Option<Food>{
    match food{
        Food::Steak => Some(Food::Steak),   // 假设对于 Steak 总有足够的调料
        Food::Salad => None,                // 假设没有足够的调料
    }
}

// 使用 map()
fn cookable_v2_map(ingredient: Ingredient){
    let meal = prepare_meal(ingredient).map(|food| {
        check_seasoning(food)
    });

    // meal 现在是 Option<Option<Food> 类型
    match meal{
        Some(Some(food)) => eat(food),
        Some(None) => println!("Not enough seasoning!"),
        None => println!("No food to prepare!"),
    }
}

// 使用 and_then()
fn cookable_v2_and_then(ingredient: Ingredient){
    let meal = prepare_meal(ingredient).and_then(|food| {
        check_seasoning(food)
    });

    match meal {
        Some(food) => eat(food),
        None => println!("No food to prepare!"),
    }
}

fn eat(food: Food){
    println!("Eating {:?}", food);
}

fn main(){
    cookable_v2_map(Ingredient::Meat);
    cookable_v2_and_then(Ingredient::Vegetable);
}