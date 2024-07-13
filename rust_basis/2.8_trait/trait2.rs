/* 实现 trait 特性的一些尝试 */
/*
报错: binary operation `>` cannot be applied to type `T`
consider restricting type parameter `T` 
std::cmp::PartialOrd

实际上 Rust Course 的介绍并不完全
因为在执行了添加特征 std::cmp::PartialOrd 之后，仍然显示出报错，
例如这句执行: let mut largest = list[0];
看似是简单的赋值操作，类似于将 y 赋值给 x, 但如果 x, y 的结构非常复杂，不仅包含在栈上分配的数据，还有在堆上分配的数据
以及类似 C 结构中的指针以及所指向的内存区域
编译器无法明确如何执行赋值操作？
选择深拷贝还是浅拷贝，如果仅仅复制指针的地址，而不选择重新 malloc 一块区域放置堆上的数据
实际上我们会有 x 和 y 同时拥有指向某一块内存的指针
带来后续的不确定性 （深浅拷贝的二次释放问题）

所以会询问: 是否在赋值的时候，具有 Copy 特性，可以直接使用类似浅拷贝的方式完成赋值，而不出错

这里推荐使用比较完全的方法，为 list: &[T] 添加满足 Copy 特性的限制
*/
fn largest<T : std::cmp::PartialOrd + Copy>(list: &[T]) -> T{
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest{
            largest = item;
        }
    }
    largest
}

fn implement_trait_for_array_slice(){
    let number_list = [1, 4, 6, 9, 18, 3, 2];
    let character_list = ['a', 'c', 'd', 'q', 'z', 'e'];

    println!("the largest in number_list: {}", largest(&number_list));
    println!("the largest in character_list: {}", largest(&character_list));
}

/*
error: cannot add `T` to `T`
std::ops::Add<Output = T>
*/
fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T{
    a + b
}

fn implement_trait_for_add(){
    println!("add i8 : {}", add(1i8, 2i8));
    println!("add i32: {}", add(10i32, 20i32));
    println!("add f64: {}", add(1.23, 2.32));
}

// 在结构体中使用泛型
// 第一种情形: struct Point<T> {x: T, y: T} 要求两个成员必须是同一类型
// 第二种情形: 需要兼容不同类型的 x 和 y, 则设置两个泛型 struct Point<T, U>{x: T, y: U}
struct Point<T, U>{
    x: T,
    y: U,
}

fn using_trait_in_struct(){
    let integer_point = Point{ x: 5, y :5};
    let float_point = Point{x: 1.5, y: 2.5};
    let string_point = Point{x: "North 3.8".to_string(), y: "East 13.79".to_string(),};

    println!("integer_point: x = {}, y = {}", integer_point.x, integer_point.y);
    println!("float_point: x = {}, y = {}", float_point.x, float_point.y);
    println!("string_point: x = {}, y = {}", string_point.x, string_point.y);

    let different_member_point = Point{x: 1.345, y: "At the beginning".to_string()};
    println!("different_member_point: x = {}, y = {}", different_member_point.x, different_member_point. y);
}

// 在方法中使用泛型
struct SingleModulePoint<T>{
    x: T,
    y: T,
}

impl<T> SingleModulePoint<T>{
    fn x(&self) -> &T{
        &self.x
    }

    fn y(&self) -> &T{
        &self.y
    }
}

// 实现该结构体方法中的泛型 impl<T> Point<T>{}
fn using_trait_in_method_single_module(){
    let single_integer = SingleModulePoint{x: 10, y: 20};
    let single_character = SingleModulePoint{x: 'h', y: 'z'};

    println!("single_integer: x = {}, y = {}", single_integer.x(), single_integer.y());
    println!("single_character: x = {}, y = {}", single_character.x(), single_character.y());
}

// 为多种泛型的结构体实现方法，同时方法中也可以定义其他类型的泛型
struct MultiModulePoint<T,U>{   // 我们定义的结构体就是 MultiModulePoint<T,U> 而不是 MultiModulePoint
    x: T,
    y: U,
}

impl<T,U> MultiModulePoint<T,U>{
    fn mixup<V,W>(self: MultiModulePoint<T,U>, other: MultiModulePoint<V,W>) -> MultiModulePoint<T,W>{
        MultiModulePoint{
            x: self.x,
            y: other.y,
        }
    }
}

// 这个例子以及 mixup 的实现，对于理解泛型在结构体中的实现，以及泛型在方法中的实现都有效
fn using_trait_in_method_multi_module(){
    let p1 = MultiModulePoint{x: 10, y: 'c'};
    let p2 = MultiModulePoint{x: 17.25, y: "Point from the other side"};

    let p3 = p1.mixup(p2);

    println!("p3's x = {}, y = {}", p3.x, p3.y);    // 理论上, p3.x = 10, p3.y = "Point from the other side"
}

fn main(){
    implement_trait_for_array_slice();      // 函数中使用泛型: 此时泛型可以用作数组切片的处理
    implement_trait_for_add();              // 函数中使用泛型: 此时泛型可以作为相加类型的处理

    using_trait_in_struct();                // 结构体中使用泛型

    using_trait_in_method_single_module();  // 方法中使用泛型: 结构体只有一种泛型

    using_trait_in_method_multi_module();   // 方法中使用泛型: 结构体中有多种泛型，同时实现的方法中也可以定义多种泛型
}