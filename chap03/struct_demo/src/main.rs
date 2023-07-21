#[derive(Debug)]
struct Person {
    // C语言风格结构体
    name: String,
    age: u8,
}

// 元组结构体
struct Pair(i32, f32);

// 单元结构体
struct Unit;

// 带两个字段的结构体
struct Point {
    x: f32,
    y: f32,
}

// 结构体可以作为另一个结构体的字段
#[allow(dead_code)]
struct Rectangle {
    // 在空间中给定左上角和右下角在空间中的位置来指定矩形
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    let name = String::from("Peter");
    let age = 21;
    let peter = Person { name, age };

    // 以Debug方式打印
    println!("{:?}", peter); // Person { name: "Peter", age: 21 }
    println!("name: {}, age: {}", peter.name, peter.age); // name: Peter, age: 21

    // 实例化结构体point
    let point: Point = Point { x: 10.3, y: 0.4 };

    // 访问Point字段
    println!("Point coordinates: ({}, {})", point.x, point.y); // Point coordinates: (10.3, 0.4)

    // 使用结构体更新语法创建新的point, 可以用到之前的point字段
    let bottom_right = Point { x: 5.2, ..point }; // second point: (5.2, 0.4)

    // bottom_right.y与point.y一样，因为这个字段是从point中来的
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // 使用let绑定来解构point
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let _rectangle = Rectangle {
        // 结构体实例化也是一个表达式
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right, // 可以省略为bottom_right: bottom_right
    };

    // 实例化一个单元结构体
    let _unit = Unit;

    // 实例化一个元组结构体
    let pair = Pair(1, 0.1);

    // 访问元组结构体
    println!("pair contains {:?} and {:?}", pair.0, pair.1); // pair contains 1 and 0.1

    // 解构一个元组结构体
    let Pair(int, dec) = pair;

    println!("pair contains {:?} and {:?}", int, dec) // pair contains 1 and 0.1
}
