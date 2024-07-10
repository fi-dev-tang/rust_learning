/*
正确的实现类的设置
*/
struct Car{
    carspeed: Option<i32>,
}

impl Car{
    fn get_in(&mut self, new_speed: Option<i32>) -> Result<(), &'static str>{
        if self.carspeed.is_some(){
            panic!("Can't assign already setted carspeed");
        }
        self.carspeed = new_speed;
        Ok(())
    }
}


fn main(){
    let mut new_car = Car{
        carspeed: None,
    };

    new_car.get_in(Some(999)).unwrap();

    println!("new car's speed: {:?}", new_car.carspeed);
}