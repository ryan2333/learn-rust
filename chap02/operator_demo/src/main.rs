fn main() {
    // 整数相加
    println!("1 + 2 = {}", 1u32 + 2);

    // 整数相减
    println!("1 - 2 = {}", 1i32 - 2);
    // print!("1 - 2 = {}", 1u32 - 2);

    // 短路bool逻辑
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // 位运算
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {:04b}", 1u32 << 5);
    println!("0x80 >> 2 is {:04b}", 0x80 >> 2);

    // 下划线改善数字的可读性
    println!("One million is written as {}", 1_000_000u32)
}
