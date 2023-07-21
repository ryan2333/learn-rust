use std::fmt;

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int, boolean) = pair;
    (boolean, int)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

fn transpose(m: Matrix) -> Matrix {
    let a = m.0;
    let b = m.2;
    let c = m.1;
    let d = m.3;
    Matrix(a, b, c, d)
}

fn main() {
    // 声明包含各种不同类型的元组
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    // 通过下标访问元组的值
    println!("first value: {}", long_tuple.0);
    println!("second value: {}", long_tuple.1);

    // 元组也可充当元组的元素
    let tuple_of_tuples = ((1u8, 2u16), (-1i8, -2i16), 0.1f32);

    // 元组也可以打印
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // 太长的元组无法打印
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("{:?}", too_long_tuple); 元组太长，报错

    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("reverse pair is {:?}", reverse(pair));

    // 创建单元素元组需要一个额外的逗号，为了和被括号包含的字面量作区分
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    // 元组可以被解构
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Debug: \n{:?}", matrix);
    println!("Display: \n{}", matrix);
    println!("Matrix: \n{}", matrix);
    println!("Transpose: \n{}", transpose(matrix));
}
