fn main() {
    // 通常情况下，`{}` 会被 任意变量内容所替换，变量内容会转换为字符串
    println!("{} days", 31); // 31 days

    // 31不加后缀，自动转换为i32类型，可以添加后缀来改变类型：31i64
    // :? 输出带类型
    println!("{:?} {:?}", 31i64, "days"); // 31 "days"

    // 变量替换字符串的多种写法
    // 1. 位置参数
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob"); // Alice, this is Bob. Bob, this is Alice

    // 2. 命名参数
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jump over"
    ); // the quick brown fox jump over the lazy dog

    // 可以在`:`后面指定特殊的格式
    // b: 二进制
    println!("{} of {:b} people know binary, the other half don't", 1, 2); // 1 of 10 people know binary, the other half don't

    // 格式化打印
    // 1. 右对齐，左边默认被空格
    println!("{number:>width$}", number = 1, width = 6); // "     1"

    // 2. 右对齐，左边补0
    println!("{number:0>width$}", number = 1, width = 6); // "000001"

    // 创建一个结构体，包含单个`i32`
    struct Structure(i32);

    // 打印结构体需要使用更复杂的方式，下面的方式无法运行
    // println!("This struct `{}` won't print...", Structure(3));

    // println 会检查使用到的参数数量是否正确
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // 控制小数位数
    let pi = 3.141592;
    println!("Pi is roughly {:.3?}", pi); // Pi is roughly 3.142
}
