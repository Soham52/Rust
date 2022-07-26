use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() 
{
    println!("Guess the number!");

    let secret_num = rand::thread_rng().gen_range(1..=100);

    loop 
    {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Could not read Line!");

        let guess: u32 = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You Guessed : {guess}");

        match guess.cmp(&secret_num) 
        {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => 
            {
                println!("You Win!!");
                break;
            }
        }
    }
}
