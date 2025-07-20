use rand::Rng;
//genrate random no.

use std::io;
//input functions

//compare
use std::cmp::Ordering;
fn main() {
    println!("Welcome to the guessing game");

    let secret_number = rand::thread_rng().gen_range(1..11);
    //thread_rng -returns a random no generator, gen_range -a method to generate a random no. within the specified range

    let mut attempts = 0;
    //record the no. of attempts

    loop {
        println!("Guess a number");
        let mut guess = String::new();
        //take multiple user inputs

        //i/p in terminal
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        //read the user input. bridge between the keyboard and the program, stdin is a function and read_line is a method

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("Give correct input");
                continue;
            }
        };
        //remove extra whitespaces and format into u32

        attempts += 1;
        //after parsing the no. of attempts increases by 1

        if guess < 1 || guess > 10 {
            println!("Out of range");
            continue;
        }

        //comparision
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small");
                if attempts > 5 {
                    println!("you have triied {} times", attempts);
                }
            }
            Ordering::Greater => {
                println!("That's huge");
                if attempts > 5 {
                    println!("you have triied {} times", attempts);
                }
            }
            Ordering::Equal => {
                println!("Correct guess");
                println!("you have tried {} times", attempts);
                break;
            }
        }
    }
}
