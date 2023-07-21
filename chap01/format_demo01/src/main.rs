use std::fmt;

struct City {
    name: &'static str, // 城市名称
    lat: f32,           // 纬度
    lon: f32,           // 经度
}

impl fmt::Display for City {
    // f 是一个缓冲区(buffer)， 此方法必须将格式化后的字符串写入其中
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // write!和format!类似，但它会将格式化后的字符串写入一个缓冲区，即f中
        write!(
            f,
            "{}: {:.3}{}; {:.3}{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// 实现Display
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // :X表示使用16进制
        // :02表示不足两位前面补0
        writeln!(
            f,
            "RGB ({red}, {green}, {blue}) 0x{red:02X}{green:02X}{blue:02X}",
            red = self.red,
            green = self.green,
            blue = self.blue
        )
    }
}

fn main() {
    for city in [
        City {
            name: "Dublin",
            lat: 53.347778,
            lon: -6.259722,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        },
    ]
    .iter()
    {
        println!("{}", *city)
    }

    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ]
    .iter()
    {
        println!("Debug {:?}", *color);
        println!("Display: {}", *color)
    }
}
