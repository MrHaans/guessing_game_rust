use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    println!("Guess the number.");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {

    println!("\nInput your number : ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
    .expect("Cant read user input!");

    let guess : u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("Your input is : {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        };
    }

    println!("\nSecret number is : {}", secret_number);
}