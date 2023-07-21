use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 使用元组的下标获取值，并创建一个vec的引用
        let vec = &self.0;

        // 使用?或try!来返回错误
        write!(f, "[")?;
        // try!(write!(f,"[")); // 2018已经移除

        for (counter, v) in vec.iter().enumerate() {
            if counter != 0 {
                // 给除了第一个元素前加上逗号
                write!(f, ", ")?;
            }
            // 写入下标和数字
            write!(f, "{}: {}", counter, v)?;
        }
        // 添加配对中括号
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3, 4]);
    println!("{}", v); // [0: 1, 1: 2, 2: 3, 3: 4]
}
