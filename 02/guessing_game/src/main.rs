// 导入 std 库中的 io 模块
use std::io;

fn main() {
    println!("Guess the number");
    println!("Please input your guess.");

    let mut guess = String::new(); // 声明一个可变变量,并返回 String 的一个实例

    let  apples = 5; // 声明一个不可变变量


    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("apples are {apples}");
    println!("You guessed: {guess}");
}
