// 使用use导入fmt模块，使fmt::Display可用
use std::fmt;

// 带有两个数字的结构体。推导出debug，以便与Display对比
#[derive(Debug)]
struct MinMax(i64, i64);

// 实现MinMax的Display
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 使用self.number来表示名个数据
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 自定义格式，只显示x,y的值
        write!(f, "x:{}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Plural {
    real: f64,
    imag: f64,
}

impl fmt::Display for Plural {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures: ");
    println!("Display: {}", minmax); // Display: (0, 14) 使用了自定义的格式
    println!("Debug: {:?}", minmax); // Debug: MinMax(0, 14)

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);
    println!(
        "The big range is {big} and the small is {small}",
        small = small_range,
        big = big_range
    );

    let point = Point2D { x: 3.3, y: 4.4 };
    println!("Compare points: ");
    println!("Display: {}", point); // Display: x:3.3, y: 4.4
    println!("Debug: {:?}", point); // Debug: Point2D { x: 3.3, y: 4.4 }

    let structure = Structure(80);

    println!("Compare Structure: ");
    println!("Display: {}", structure); // Display: 80
    println!("Debug: {:?}", structure); // Debug: Structure(80)

    let num = Plural {
        real: 3.3,
        imag: 7.2,
    };
    println!("Compare Plural: ");
    println!("Display: {}", num); // Display: 3.3 + 7.2i
    println!("Debug: {:?}", num); // Debug: Plural { real: 3.3, imag: 7.2 }
}
