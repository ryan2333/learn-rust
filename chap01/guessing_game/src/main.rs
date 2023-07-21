use rand::Rng;
use std::cmp::Ordering;
use std::io; // 导入标准库

// main函数：程序入口
fn main() {
    println!("Guess the number...");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your number...");
        let mut guess: String = String::new();
        // 未在文件开头引入时，使用stdin::io::Stdin
        io::stdin()
            // 获取用户输入，存入变量guess
            // &表示引用，引用默认是不可变的，需要添加mut，所以是&mut guess而不是&guess
            .read_line(&mut guess)
            .expect("Failed to read line!!!");

        // let guess: u32 = guess.trim().parse().expect("Please input a number...");
        // println!("secret number is {secret_number}, you guessed: {guess}");

        // 处理无效输出，将expect替换为match, 使程序继续运行，而不是崩溃退出
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,     // 转换正常返回转换后的值
            Err(_) => continue, // 转换失败，执行下次循环
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small..."),
            Ordering::Greater => println!("Too big..."),
            Ordering::Equal => {
                println!("You win...");
                break;
            }
        }
    }
}
