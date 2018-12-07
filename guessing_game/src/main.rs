use std::io;

fn main() {
    println!("Guess the number !");
    println!("Please input your guess.");

    // 可変の場合はmutを付ける
    let mut guess = String::new();

    // ioライブラリから標準入力の受け取り
    // useしない場合はフルパスを書く
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {}", guess);
}
