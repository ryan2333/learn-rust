// 用于隐藏对未使用代码的警告
#![allow(dead_code)]
enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    // 显示在use各个名称，使它们直接可用，而不需要指定它们来自Status
    use Status::{Poor, Rich};

    // 自动的use Work内部的各个名称
    use Work::*;

    // Poor 等价于Status::Poor
    let status = Poor;

    // Civilian 等价于Work::Civilian
    let work = Civilian;

    match status {
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }
    /**
     * The poor have no money...
     * Civilians work!
     */
}
