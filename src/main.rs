extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
  println!("Guess the number!");
  let secret_number = rand::thread_rng().gen_range(1, 101);
  println!("The secret number is: {}", secret_number); //秘 密 の 数 字 は 次 の 通り: {}
  
  println!("Please input your guess.");
  
  let mut guess = String::new();
  
  io::stdin().read_line(&mut guess)
  .expect("Failed to read line");
  
  let guess: u32 = guess.trim().parse()
  .expect("Please type a number!"); //数 値 を 入 力 し て く だ さい！

  println!("You guessed: {}", guess);

  
  match guess.cmp(&secret_number) {
  Ordering::Less => println!("Too small!"), //小 さ す ぎ！
  Ordering::Greater => println!("Too big!"), //大 き す ぎ！
  Ordering::Equal => println!("You win!"), // や っ た ね！
  }
}