

// Guess the game
// take input from user
// parse the input
// return if it matches the random number
// continous for loop

use rand::prelude::*;

fn gen_random_number() -> u8 {
    let n:u8 = rand::thread_rng().gen_range(0..50);
    return n;
}

fn main() {
    println!("Welcome to game");
    let number:u8= gen_random_number();
    print!("{}", number);
}