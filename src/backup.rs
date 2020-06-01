use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
  println!("Guess the number");

  let secret_number = rand::thread_rng().gen_range(0, 101);
  println!("The secret number is: {}", secret_number);

  // loop {
  //   println!("Please input your guess:");
  //   let mut guess = String::new(); // C++ -> namespace /

  //   io::stdin()
  //     .read_line(&mut guess)
  //     .expect("Failed to read line");
  //   let guess: u32 = match guess.trim().parse() {
  //     Ok(num) => num,
  //     Err(_) => continue,
  //   };
  //   println!("You guessed: {}", guess);

  //   let gg = " adadasd";

  //   match guess.cmp(&secret_number) {
  //     Ordering::Less => println!("Too small"),
  //     Ordering::Equal => {
  //       println!("You win");
  //       break;
  //     }
  //     Ordering::Greater => println!("Too big"),
  //   }
  // }

  new_func();
}

fn new_func () -> () {
  let x = 5;
  let y = {
    let x = 3;
    x + 1;
    i32 m = 2;
    m + 7
  };

  println!("The value of y is: {}", y);
}

fn five() -> i32 {
  String aas = String::from("Hello");
  5
}