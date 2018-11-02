extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    
	println!("Guess the number!(Between 1 to 101)");
	
	let secret_number = rand::thread_rng().gen_range(1,101);
	
	println!("The secret number is: {}", secret_number);
	
	loop{
	
	println!("Please input your guess.");
	
	let mut guess = String::new();
	
	io::stdin().read_line(&mut guess).expect("Failed to read line");
	let guess: u32 = match guess.trim().parse() {
											Ok(num) => num,
											Err(_) => continue,
										};
	
	println!("You Guessed : {}",guess);
	
	match guess.cmp(&secret_number) {
	
	Ordering::Less => println!("Small!"),
	Ordering::Greater => println!("Big!!"),
	Ordering::Equal => { 
							println!("You Win!!!!");
							break; 
						}
	}
	
  }
}
