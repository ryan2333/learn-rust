fn main() {
    // 声明变量时指定类型
    let logical: bool = true;

    // 常规方式指定变量类型
    let a_float: f64 = 1.0;

    // 后缀方式指定变量类型
    let a_int = 6i32;

    // 使用默认类型
    let default_float = 3.0; // f64
    let default_int = 7; // i32

    // 通过上下文自动推断类型
    let mut inferred_type = 12; // 根据下一行推断为i64类型
    inferred_type = 4294967296i64;

    // 可变变量，值可以改变
    let mut mutable = 12;
    mutable = 21;

    // 报错！变量类型无法改变
    // mutable = true;

    // 使用遮蔽(shadow)来覆盖前面的变量
    let mutable = true;
}
