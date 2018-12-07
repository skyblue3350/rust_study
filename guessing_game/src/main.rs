extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number !");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is {}", secret_number);

    println!("Please input your guess.");

    // 可変の場合はmutを付ける
    let mut guess = String::new();

    // ioライブラリから標準入力の受け取り
    // useしない場合はフルパスを書く
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {}", guess);

    // 比較用に型を揃える
    let guess: u32 = guess.trim().parse().expect("please type number");

    match guess.cmp(&secret_number) {
        Ordering::Less    => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal   => println!("You win"),
    }
}
