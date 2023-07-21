#![allow(dead_code)]

// 拥有隐式辨别值的enum，从0开始
enum Number {
    Zero,
    One,
    Two,
}

// 拥有显示辨别值的enum
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    // enum 可以转成整型
    println!("zero is {}", Number::Zero as i32); // zero is 0
    println!("one is {}", Number::One as i32); // one is 1

    println!("roses are #{:06x}", Color::Red as i32); // roses are #ff0000
    println!("roses are #{:06x}", Color::Blue as i32); // roses are #0000ff
}
