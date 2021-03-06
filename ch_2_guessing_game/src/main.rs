use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    //Loop over
    loop {
        println!("Please input your guess.");

        //Create mutable binding with a string type
        let mut guess = String::new();

        //Read user input to a guess mut address
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //Parse string, Convert  string to an u32 integer number 
        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                //break from the loop
                break;
            }
        }
    }
}
