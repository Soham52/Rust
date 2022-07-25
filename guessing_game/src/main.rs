use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main()
{
    println!("Guess the number!");

    let secret_num = rand::thread_rng().gen_range(1..=100);
    println!("The Secret number is: {secret_num}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().
        read_line(&mut guess).
        expect("Could not read Line!");

    let guess: u32 = guess.trim().parse().expect("Please Enter a Number");

    println!("You Guessed : {guess}");

    match guess.cmp(&secret_num)
    {
        Ordering::Less => println!("Too Small!"),
        Ordering::Greater => println!("Too Big!"),
        Ordering::Equal => println!("You Win!!"),
    }
}
