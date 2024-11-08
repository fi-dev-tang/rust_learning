/*
Option & unwrap
*/
fn give_common(gift: Option<&str>){
    match gift{
        Some("snake") => println!("Yuck, snake"),
        Some(inner) => println!("I love {}", inner),
        _ => println!("Empty?"),
    };
}


fn give_princesses(gift: Option<&str>){
    let inner = gift.unwrap();

    if inner == "snake"{
        panic!("Yuck, AAAAAAAaaaaaa");
    }

    println!("I love {}s", inner);
}

fn main(){
    let food = Some("cabbage");
    let sweet = Some("snake");
    let void = None;

    give_common(food);
    give_common(sweet);
    give_common(void);

    let bird = Some("robin");
    let nothing = None;

    give_princesses(bird);
    give_princesses(nothing);
}