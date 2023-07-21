struct Rectangle {
    width: f64,
    height: f64,
}

fn rect_area(rect: Rectangle) -> (f64, f64) {
    // 解构结构体
    let Rectangle { width, height } = rect;

    // 解构并重命名变量名称
    let Rectangle {
        width: x,
        height: y,
    } = rect;
    (width * height, x * y)
}

fn main() {
    let rect = Rectangle {
        height: 3.5,
        width: 2.8,
    };

    let (x, y) = rect_area(rect);

    println!("rect area is {:.2} {:.2}", x, y)
}
