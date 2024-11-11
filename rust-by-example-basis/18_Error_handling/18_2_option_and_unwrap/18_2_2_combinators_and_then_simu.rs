/* 
模拟 and_then 的使用，
一个很好的做法是: 如果前面调用的 Option 是 None, 闭包不会被执行
*/
#[derive(Debug)]
enum Food{
    Steak,
    CordonBleu,
    Sushi,
}

#[derive(Debug)]
enum Day{
    Monday,
    Tuesday,
    Friday,
}

fn have_ingredients(food: Food) -> Option<Food>{
    match food {
        Food::Sushi => None,
        _ => Some(food),
    }
}

fn have_recipe(food: Food) -> Option<Food>{
    match food{
        Food::CordonBleu => None,
        _ => Some(food),
    }
}

fn cookable_v1(food: Food) -> Option<Food>{
    match have_ingredients(food) {
        None => None,
        Some(food) => {
            match have_recipe(food) {
                None => None,
                Some(food) => Some(food),
            }
        }
    }
}

fn cookable_v2(food: Food) -> Option<Food>{
    have_ingredients(food).and_then(have_recipe)
}

fn cookable_v3(food: Food) -> Option<Food>{
    have_ingredients(food).and_then(|food| {
        have_recipe(food)
    })
}

fn eat_v1(food: Food, day: Day){
    match cookable_v1(food) {
        None => println!("Can't have food to eat!"),
        Some(food) => println!("have {:?} to eat on {:?}", food, day),
    }
}

fn eat_v2(food: Food, day: Day){
    match cookable_v2(food) {
        None => println!("Can't have food to eat!"),
        Some(food) => println!("have {:?} to eat on {:?}", food, day),
    }
}

fn eat_v3(food: Food, day: Day){
    match cookable_v3(food) {
        None => println!("Can't have food to eat!"),
        Some(food) => println!("have {:?} to eat on {:?}", food, day),
    }
}

fn main(){
    eat_v1(Food::Sushi, Day::Monday);
    eat_v1(Food::Steak, Day::Monday);
    eat_v1(Food::CordonBleu, Day::Monday);

    eat_v2(Food::Sushi, Day::Tuesday);
    eat_v2(Food::Steak, Day::Tuesday);
    eat_v2(Food::CordonBleu, Day::Tuesday);

    eat_v3(Food::Sushi, Day::Friday);
    eat_v3(Food::Steak, Day::Friday);
    eat_v3(Food::CordonBleu, Day::Friday);
}