/*
实现 fmt::Display 功能
展示 RGB (128, 255, 90) 0x80FF5A
RGB 计算公式: RGB = (R * 65536) + (G * 256) + B
*/
use std::fmt::{Display};
use std::fmt;   
/* 实际上也可以写成 use std::fmt::{self, Formatter, Display}, 打印时按照 Formatter 进行类型设置 */

struct City{
    name: &'static str,
    lat: f64,
    lon: f64,
}

struct Color{
    red: i32,
    green: i32,
    blue: i32,
}

/*
City 的打印格式为：城市名称 - 所在的纬度(N or S) - 所在的经度(E or W)
*/
impl Display for City{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
        /*
        这里是报错出现的问题，传递的是 &self, 
        所以涉及的 lat 和 lon 实际上是 &f64, 不能直接和 f64 比较
        */
        let lat_c = if *&self.lat >= 0.0 {'N'} else {'S'};
        let lon_c = if *&self.lon >= 0.0 {'E'} else {'W'};

        write!(f, "{} locate at {:.3}°{}, {:.3}°{}", &self.name, &self.lat, lat_c, &self.lon, lon_c)
    }
}

impl Display for Color{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
        let sum = (&self.red * 65536) + (&self.green * 256) + &self.blue;
        write!(f, "RGB ({}, {}, {}) 0x{:0>6X}", &self.red, &self.green, &self.blue, sum)
    }
}

fn main(){
    for city in [
        City{name: "Beijing", lat: 57.62438, lon: 98.15746},
        City{name: "Pairs", lat: 134.92847, lon: 25.86740},
        City{name: "Astrualia", lat: -134.67753, lon: -63.4892}
    ] {
        println!("{}", city);
    }

    for color in [
        Color{red: 128, green: 255, blue: 90},
        Color{red: 0, green:3, blue: 254},
        Color{red: 0, green:0, blue: 0}
    ]{
        println!("{}", color)
    }
}