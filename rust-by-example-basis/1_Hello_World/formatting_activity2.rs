/*
重新实现 City 和
之前写法的错误，本来已经传递了引用&, 现在变成传递引用的引用 
相当于 fn fmt(&self) 这里的 & 表示取地址运算符 , self 已经是 &Color 或者 &City 类型
所以在后续访问过程中，直接使用 self. 即可， Rust 自动处理解引用的过程
*/
use std::fmt::{self, Formatter, Display};

struct City{
    name: &'static str,     // 城市名称
    lat: f32,               // 纬度
    lon: f32,               // 经度
}

struct Color{
    red: i32,
    green: i32,
    blue: i32,
}

impl Display for City{
    fn fmt(&self, f:&mut Formatter) -> fmt::Result{
        let lat_c = if self.lat >= 0.0 {'N'} else {'S'};
        let lon_c = if self.lon >= 0.0 {'E'} else {'W'};

        write!(f, "{}: {:.3}°{} {:.3}°{}", self.name, self.lat, lat_c, self.lon, lon_c)
    }
}

impl Display for Color{
    fn fmt(&self, f:&mut Formatter) -> fmt::Result{
        let result = (self.red * 65536) + (self.green * 256) + self.blue;
        write!(f, "RGB ({},{},{}) 0x{:0>6X}", self.red, self.green, self.blue, result)
    }
}

fn main(){
    for city in [
        City{name: "Beijing", lat:176.9563, lon: 24.5724},
        City{name: "Singapore", lat:-24.73924, lon: 138.247924},
        City{name: "Austrulia", lat: -98.374924, lon: -76.35843},
    ]{
        println!("{}", city);
    }

    for color in [
        Color{red: 128, blue: 90, green: 255},
        Color{red: 0, blue:254, green: 3},
        Color{red:0, blue:0, green: 0}
    ]{
        println!("{}", color)
    }
}