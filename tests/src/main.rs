use std::io;

fn main()
{
    let a = [1, 2, 3, 4, 5];
    let mut index = String::new();
    
    println!("Enter an index: ");

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line!");

    let index: usize = index
        .trim()
        .parse()
        .expect("Not an integer!");
    
    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
