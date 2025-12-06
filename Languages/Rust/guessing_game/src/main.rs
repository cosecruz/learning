//guessing_game
use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main(){
    println!("Welcome to the guessing game!");

    let secret_number = rand::rng().random_range(1..=100);
    //let mut tries = 0;

    loop{
        println!("PLease enter your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Read line");

        let guess: i32= match guess.trim().parse() {
            Ok(num)=> num,
            Err(_)=> continue
        };

        println!("Your guess is {guess}");

        match guess.cmp(&secret_number){
            Ordering::Less=> println!("You guess is less"),
            Ordering::Equal=> {
                println!("You guess is correct");
                break
            },
            Ordering::Greater=> println!("You guess is greater"),

        }
    }

}
