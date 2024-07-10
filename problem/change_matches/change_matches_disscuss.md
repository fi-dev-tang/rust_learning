# 问题描述和起因
> 问题描述:
> 在 《Rust语言圣经(Rust Course)》的 2.6.4 节全模式列表中，出现使用嵌套的 _ 忽略部分值。

展示代码如下:
```rust
let mut setting_value = Some(5);
let new_setting_value = Some(10);

match(setting_value, new_setting_value){
    (Some(_), Some(_)) => {
        println!("Can't overwrite an existing customized value");
    }
    _ => {
        setting_value = new_setting_value;
    }
}

println!("setting is {:?}", setting_value);
```
希望将这部分核心代码写成函数，并且参数以引用的方式进行传递。
在具体写函数的过程中，出现了诸多编译器的报错。

# 讨论过程
```rust
fn customized_change1<'a>(
    mut setting_value: &'a mut Option<i32>,
    new_setting_value: &'a mut Option<i32>,
){}
```
这个函数接受两个参数，从 C 语言视角，或者从内存视角看，这两个参数都是指针(8 bytes)。
它们指向一个 Option<i32>, 可为空的 int32。

接着从 rust 语义看，
```rust
&'a mut Option<i32>
```
这里的 mut 表示我们可以通过这个指针去修改被指向的目标。

然后，第一个参数 setting_value 它的前面有一个 mut, 说明这个变量可修改，即，可以指向新的位置。
```rust
fn customized_change1<'a>(
    mut setting_value: &'a mut Option<i32>,
    new_setting_value: &'a mut Option<i32>,
)
```
先不考虑 match(...) 括号中的东西。
```rust
setting_value = new_setting_value;
```
这条语句表示，把第二个参数的值赋给第一个参数，也就是函数内临时变量 setting_value 指向了 new_setting_value 指向的位置。
没有达成函数的预期目标，它不会修改函数外部的任何东西。
实际上，直接去掉参数类型中的 mut 也不会报错。

通过指针修改被指向的位置，需要这样写:
```rust
let mut num: Option<i32> = Some(123);
let ptr: &mut Option<i32> = &mut num;

*ptr = Some(456);
```

从内存角度看，Rust 的引用和 C++ 的别名都是指针；Rust 和 C++ 赋予了不同的语义。

如果想让 Rust 中的引用直接修改函数外部的值，应该写成:
```rust
fn change_settings(current: &mut Option<i32>, new: Option<i32>){
    if current.is_some() {
        panic!("无法修改已自定义的设置。");
    }

    *current = new;
}
```
现在第一个参数是一个指针，第二个参数是 Option<i32>, 它们的大小分别是 8 Bytes 和 8 Bytes。
如果设置项不是 i32, 而是一个较大的东西，那么这样写不合适。现在 i32 只有 4 Bytes, 被 Option 包装一下为 8 Bytes，很小，因此直接传参。

## 小节
- 每个变量必有其 type, 每个 type 必有其确定大小(字节数)。
- &mut T 这样的 type, 实际是指针，可以通过这个变量去修改被指向的东西。
- let mut xxx 这样的声明，说明这个变量，它的内容是可以被修改的。

举例来说，拿到这个 ptr 可以修改 num。但这个 ptr 不能指向新的东西:
```rust
let mut num: Option<i32> = Some(123);
let ptr: &mut Option<i32> = &mut num;

*ptr = Some(456);
```

而这个 let mut ptr 可以指向新的东西，但是被指向的东西是只读的；
```rust
let num: Option<i32> = Some(123);
let num2: Option<i32> = Som(456);

let mut ptr: &Option<i32> = &num;
ptr = &num2;
```

## 设置类的代码写法
```rust
use anyhow::{bail, Result};

struct Settings{
    carspeed: Option<i32>,
}

impl Settings {
    fn get_in_car(&mut self, carspeed: i32) -> Result<()> {
        if self.carspeed.is_some() {
            bail!("Forbid get in the car!");
        }
        self.carspeed = Some(carspeed);
        Ok(())
    }
}

fn main(){
    let mut s = Settings{
        carspeed: None
    };

    s.get_in_car(999).unwrap();
}
```