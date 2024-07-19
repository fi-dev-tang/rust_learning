/*
一个综合复习，检测对泛型约束的理解，对闭包函数的理解，对 Cacher 功能的实际理解
核心逻辑是: 如果 Cacher 内存储的值是一个空值，那么直接使用闭包函数对传入的参数进行运算，将运算结果存入 Cacher 的 value 中
否则直接从 Cacher 的 value 中取值, 
达成的效果, 对于由同一个 闭包函数 new 出来的 Cacher, 只计算一次函数的值并存储，不会更新出新的值
*/
fn example1(){
    struct Cacher<T: Fn(u32) -> u32>{
        calculation: T,
        value: Option<u32>,
    }

    impl<T: Fn(u32) -> u32>  Cacher<T>{
        fn new(calculation:T) -> Cacher<T>{
            Cacher{
                calculation: calculation,
                value: None,
            }
        }

        fn return_value(&mut self, arg: u32) -> u32{
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                },
            }
        }
    }

    let mut cacher = Cacher::new(|x| x + 1);
    assert_eq!(cacher.return_value(20), 21);
    assert_eq!(cacher.return_value(0), 21);
}

fn example2(){
    // 使用 where 进行条件限制
    struct Cacher<T>
        where T: Fn(u32) -> u32,
    {
        calculation: T,
        value: Option<u32>,
    }

    impl<T> Cacher<T>
        where T: Fn(u32) -> u32,
    {
        fn new(calculation: T) -> Cacher<T>{
            Cacher{
                calculation: calculation,
                value: None,
            }
        }

        fn return_value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                },
            }
        }
    }

    let mut cacher = Cacher::new(|x| 2 * x);
    assert_eq!(cacher.return_value(4), 8);
    assert_eq!(cacher.return_value(8), 8);
}

fn main(){
    example1();
    example2();
}