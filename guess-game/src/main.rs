// Guess game
// take input from user - done
// parse the input - done
// return if it matches the random number - done
// continous for loop - done

use rand::prelude::*;
use std::{io, process};

fn gen_random_number() -> u8 {
    let n: u8 = rand::thread_rng().gen_range(0..50);
    n
}

fn take_input() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    match stdin.read_line(&mut buffer) {
        Ok(_) => buffer.to_string().strip_suffix('\n').unwrap().to_string(),
        Err(e) => e.to_string(),
    }
}

fn fail_exit(msg: &str) -> ! {
    eprintln!("{} fool", msg);
    process::exit(1)
}

fn game_loop(number:u8) {
    loop {
        println!("Guess the number ");
        let input = match take_input().parse::<u8>() {
            Ok(n) => n,
            Err(e) => fail_exit(&e.to_string())
        };
        if input == number {
            println!("You have guessed it right. Now go!");
            break;
        } else {
            println!("Do you wish to continue y/n ?");
            let ans = take_input();
            if ans == "y" {
                continue;
            } else {
                break;
            }
        }
    }

}

fn main() {
    println!("Welcome to game");
    let number: u8 = gen_random_number();
    game_loop(number);
}
