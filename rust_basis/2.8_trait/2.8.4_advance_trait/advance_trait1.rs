/*
特征的进阶部分
*/
// 关联类型: (1) 对比关联类型和泛型(counter trait)  (2) 再实现一个 container 的 trait 体会区别
// (3) 一个非常复杂的 Address

// (1) 对比关联类型和泛型实现(counter trait)
// 关联类型实现 Counter Trait
pub trait AssociatedTypeCounter{
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

// 泛型实现 Counter Trait
pub trait GenericCounter<Item>{
    fn next(&mut self) -> Option<Item>;
}

// 定义一个用到关联类型的 CounterExample1
struct CounterEx1{
    value: u32,
}

impl AssociatedTypeCounter for CounterEx1{
    type Item = u32;
    fn next(&mut self) -> Option<u32>{
        if self.value == 0 {
            None
        }else{
            self.value += 1;
            Some(self.value)
        }
    }
}

// 定义一个用到关联类型的 CounterExample2
struct CounterEx2{
    value: String,
}

impl GenericCounter<String> for CounterEx2{
    fn next(&mut self) -> Option<String>{
        if self.value == "" {
            None
        }else{
            let mut result = format!("{}", self.value);
            result.push_str(&format!(" Again!"));
            self.value = result.clone();
            Some(result.clone())
        }
    }
}

// 两种方式，一种是关联类型实现 counter, 一种是泛型实现 counter
fn associated_vs_generic(){
    let mut associated_counter = CounterEx1{value: 13};
    println!("associated_counter:{}", associated_counter.value);        // 分成两行写，是因为读取本身采用 immutable borrow, 但 next(&mut self) 采用 mutable borrow
    println!("next: {}", associated_counter.next().unwrap());           // 发生可变借用和不可变借用出现在同一行的问题

    let mut generic_counter = CounterEx2{value: String::from("Rust counter")};
    println!("generic_counter:{}", generic_counter.value);
    println!("next:{}", generic_counter.next().unwrap());
}

fn main(){
    associated_vs_generic();                // 对比关联类型和泛型实现 counter trait
}