fn main() {
    // 这个结构不能使用 fmt::Display或 fmt::Debug来进行打印
    // 没有使用derive自动创建fmt::Debug的实现
    struct UnPrintable(i32);

    // derive属性会自动创建所需的实现，使这个struct能使用fmt::Debug打印
    #[derive(Debug)]
    struct DebugPrintable(i32);

    println!("{:?} monents in a year.", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );
    // struct DebugPrintable也可以打印，因为#[derive(Debug)]自动创建了fmt::Debug的实现
    println!("Now {:?} will print!", DebugPrintable(3));

    // 美化打印
    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // 标准打印
    println!("{:?}", peter); // Person { name: "Peter", age: 27 }

    // 通过{:#?}美化打印
    println!("{:#?}", peter);
    /*
    Person {
        name: "Peter",
        age: 27,
    }
     */
}
