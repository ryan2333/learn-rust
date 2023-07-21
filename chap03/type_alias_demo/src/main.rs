enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

// 创建一个类型别名
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

fn main() {
    // 我们可以通过别名引用每个枚举变量，避免使用以长又不方便的枚举名字
    let add = Operations::Add;
    let sub = Operations::Subtract;

    println!("3 + 2 = {}", add.run(3, 2));
    println!("3 - 2 = {}", sub.run(3, 2));
}
