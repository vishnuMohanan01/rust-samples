use rand::Rng;
use std::io;
use std::cmp::Ordering;
use colored::Colorize;

fn main() {
  println!("\n\n=================== Welcome to guessing game !================\n\n");

  let secret_number = rand::thread_rng().gen_range(0..101);
  println!("Secret number is {}:", secret_number.to_string().cyan());
  
  loop {
    println!("Input a number: ");
    let mut guess = String::new(); // will be shadowed later
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("{}", "Please enter a number!".yellow());
        continue;
      }
    };

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
      Ordering::Less => {
        println!("{}", "Too small!".red());
        continue;
      },
      Ordering::Equal => {
        println!("{}", "You win!".green());
        break;
      },
      Ordering::Greater => {
        println!("{}", "Too big!".red());
        continue;
      }
    }
  }
}
