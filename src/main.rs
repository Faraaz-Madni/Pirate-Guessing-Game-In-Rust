use std::io;
use rand::{Rng};
use std::cmp::Ordering;

fn main() {
    println!("-- Arrr, welcome to me BLAZIN' FAST Number Guessin' Game built in the Rust programmin' language! --");
    println!("> Avast! Take a shot at guessing a number, me hearty!");

    let random_number = rand::thread_rng().gen_range(1..=5);

loop {
        let mut player_guess = String::new();

        io::stdin()
        .read_line(&mut player_guess) //The & sign means we are passing something as a read-only value.
        .expect("Failed to read your guess.");

        let player_guess:u32 = match player_guess.trim().parse(){
            Ok(num) => {
                if num > 0 && num <= 5{
                num
                }
                else{
                println!("Arrrrrr! Enter a NUMBER that be bigger than nothing but smaller than 6!");
                continue;
                }
            }
            Err(_) => {
                println!("Arrrrrr! Enter a NUMBER or walk the plank! My patience has no need for ye fancy schmancy arithmetic signs either!");
                continue;
            }
        };
        println!("Aye, so ye thought ye had me game beat with the number {player_guess}? Let's take a gander...");


        // Match compares one variable against another. The two variables should be of the same type though.
        match player_guess.cmp(&random_number)
        {
            Ordering::Equal => {
                let res = "Shiver me timbers, ye guessed right! Ye win the treasure!";
                println!("{res}");
                break;
            }, 
            Ordering::Greater => {
                let res = "Ye guessed a number as big as a whale!";
                println!("{res}");
            },
            Ordering::Less => {
                let res = "Ye guessed too low, ye landlubber!";
                println!("{res}");
            }  
        }
    }

    // TODO - lose scenario
    // Ye lost, and it's time for ye to walk the plank! Off to Davy Jones' locker for an eternity!
    // println!("The number ye should've guessed be {random_number}");

}
