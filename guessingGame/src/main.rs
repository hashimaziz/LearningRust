use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut guesses = 0;
    //println!("The secret number is: {}", secret_number);
    
    loop{
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin() //accept user input
            .read_line(&mut guess)//readline from terminal and stores value in guess
            .expect("Failed to read line");//crash if an issue occurs
        
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess); // {} used as placeholders

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                guesses += 1;
                println!("You won in {} guesses!", guesses);
                break;
            }
        }
        guesses += 1;
    }
}