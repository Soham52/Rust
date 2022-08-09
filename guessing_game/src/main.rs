use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main()
{
    println!("Guess The Number!!");

    let secret_num = rand::thread_rng().gen_range(1..=100);

    loop
    {
        let mut guess = String::new();

        println!("Enter your guess: ");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");
        
        let guess: u32 = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You Guessed: {guess}");

        match guess.cmp(&secret_num) 
        {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Large!"),
            Ordering::Equal =>
            {
                println!("You Win!!");
                break;
            }
        }
    }
}
