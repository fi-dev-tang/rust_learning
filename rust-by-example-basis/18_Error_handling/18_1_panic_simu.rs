/*
panic 出现在显式的错误中
*/
fn give_princesses(gift: &str){
    if gift == "snake" {
        panic!("AAAAAAAAAAAAAAAAAAAAAAaaaaaaaaaa!!!!");
    }

    println!("I love {}s", gift);
}

fn main(){
    give_princesses("teddy bear");
    give_princesses("snake");
}