// 整数类型: 使用方法测试整数是否溢出
fn number_overflow(){
    let a: u8 = 255;
    // wrapping_* 方法在所有模式下都按照补码循环溢出规则处理，wrapping_add
    let b = a.wrapping_add(20);
    println!("{}", b);
}

// 浮点数类型: 想要数字的近似表示
/* 
因为二进制精度问题，导致了 0.1 + 0.2 并不严格等于 0.3, 它们可能在小数点 N 位后存在误差。
如果非要进行比较? 可以采用这种方式
(0.1_f64 + 0.2 - 0.3).abs() < 0.00001
具体小于多少，取决于你对精度的需求
!! 避免在浮点数上测试相等性
*/
fn float_near(){
    // 断言 0.1 + 0.2 与 0.3 相等
    // assert!(0.1 + 0.2 == 0.3); // 这里会抛出 panic 异常
    assert!((0.1_f64 + 0.2 - 0.3).abs() < 0.00001);
}

/*
对 f32 类型做加法时，0.1 + 0.2 的结果是 3e99999a, f32 下通过测试
到了 f64 类型，结果就不一样了，因为 f64 精度高很多，小数点后面发生了微小的变化。
*/
fn float_32_and_64(){
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("  0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("        0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("  0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("        0.3: {:x}", (xyz.2).to_bits());
    println!();

    assert!(abc.0 + abc.1 == abc.2);
    // assert!(xyz.0 + xyz.1 == xyz.2);  // panic
}

fn nan_test(){
    let x = (-42.0_f32).sqrt();
    // assert_eq!(x,x);  // 代码崩溃，NaN 不能用来比较

    if x.is_nan(){
        println!("undefined mathmatical behavior!");
    }
}

fn calculate_example(){
    let twenty = 20;            // 编译器会进行自动推导，给予 twenty i32 的类型
    let twenty_one: i32 = 21;   // 类型标注
    let twenty_two = 22i32;     // 通过类型后缀的方式进行类型标注: 22 是 i32 类型

    let addition = twenty + twenty_one + twenty_two;    // 只有同样类型，才能运算
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

    let one_million: i64 = 1_000_000;   // 对于较长的数字，可以用_ 进行分割，提升可读性
    println!("{}",one_million.pow(2));

    let forty_twos = [      // 定义一个 f32 数组，其中 42.0 会自动被推导为 f32 类型
        42.0,
        42f32,
        42.0_f32,
    ];

    println!("{:.2}", forty_twos[0]);   // 打印数组中第一个值，并控制小数位为2位
}

fn bit_calculate(){
    // 二进制为 0000010
    let a: i32 = 2;
    // 二进制为 0000011
    let b: i32 = 3;

    println!("(a & b) value is {}", a & b);
    println!("(a | b) value is {}", a | b);
    println!("(a ^ b) value is {}", a ^ b);
    println!("(!b) value is {}", !b);
    println!("(a << b) value is {}", a << b);
    println!("(a >> b) value is {}", a >> b);
    
    let mut a = a;
    // 注意这些计算夫除了 ! 之外都可以加上 = 进行赋值 (因为 != 要来判断不等于)
    a <<= b;
    println!("(a << b) value is {}",a);
}

fn generate_range(){
    for i in 1..=5 {
        print!("{}",i);
    }

    for i in 'a'..='z' {
        print!("{} ",i);
    }
}

fn main(){
    number_overflow();
    float_near();

    float_32_and_64();

    nan_test();
    calculate_example();

    bit_calculate();
    generate_range();
}