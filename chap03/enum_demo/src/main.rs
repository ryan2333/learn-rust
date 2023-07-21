// 该属性用于隐藏对使用代码的警告
#![allow(dead_code)]

// 创建一个enum来对web事件分类。注意变量名和类型共同指定了enum取值的种类
enum WebEvent {
    // 一个enum可以是单元结构体
    PageLoad,
    PageUnload,
    // 或一个元组结构体
    KeyPress(char),
    Paste(String),
    // 或一个普通的结构体
    Click { x: i64, y: i64 },
}

// 此函数将一个WebEvent enum作为参数， 无返回值
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // 从enum里解构了`c`
        WebEvent::KeyPress(c) => println!("pressed '{}'", c),
        WebEvent::Paste(s) => println!("pasted \"{}\"", s),
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}", x, y)
        }
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    // to_owned() 从一个字符串切片中创建一个具有所有权的String
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 15, y: 28 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed); // pressed 'x'
    inspect(pasted); // pasted "my text"
    inspect(click); // clicked at x=15, y=28
    inspect(load); // page loaded
    inspect(unload); // page unloaded
}
