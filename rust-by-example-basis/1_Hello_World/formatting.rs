/*
format!("{}", foo) -> "3735928559"
format!("0x{:X}", foo) -> "0xDEADBEEF"
format!("0o:{:o}", foo) -> "0o33653337357"
*/
use std::fmt::{self, Formatter, Display};

struct City {
    name: &'static str,
    lat: f32,   // 纬度 Latitude
    lon: f32,   // 经度 Longitude
}

impl Display for City{
    // `f` is a buffer, and this method must write the formatted string into it.
    fn fmt(&self, f:&mut Formatter) -> fmt::Result{
        let lat_c = if self.lat >= 0.0 {'N'} else {'S'};
        let lon_c = if self.lon >= 0.0 {'E'} else {'W'};

        // write! is like format!, but it will write the formatted string into a buffer(the first argument)
        write!(f, "{}: {:.3}°{} {:.3}°{}",
            self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn main(){
    for city in [
        City {name: "Dublin", lat: 53.347778, lon: -6.259722},
        City {name: "Oslo", lat: 59.95, lon: 10.75},
        City{ name: "Vancouver", lat: 49.25, lon: -123.1},
    ]{
        println!("{}", city);
    }
}