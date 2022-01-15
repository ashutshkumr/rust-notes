use std::io;
use std::cmp;
use std::io::Write;

pub fn guessing_game() {
    println!("Enter a guess (between 1..100).");

    loop {
        print!("Guess: ");
        io::stdout().flush().unwrap();

        let target = 55;
        let mut guess = String::new();

        if let Err(err) = io::stdin().read_line(&mut guess) {
            println!("could not read line, try again: {}", err);
            continue;
        }

        match guess.trim().parse::<u32>() {
            Ok(num) => {
                match target.cmp(&num) {
                    cmp::Ordering::Less => println!("too high !"),
                    cmp::Ordering::Greater => println!("too low !"),
                    cmp::Ordering::Equal => {
                        println!("matched !");
                        break;
                    }
                }
            },
            Err(err) => {
                println!("could not parse u32, try again: {}", err);
                continue;
            }
        }
    }
}