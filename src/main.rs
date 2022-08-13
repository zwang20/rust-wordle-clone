use std::{fs, io};
use std::io::BufRead;

use colored::Colorize;
use rand::seq::SliceRandom;

fn print_guess(guess: &str, word: &str) {
    for i in 0..5 {
        if guess.as_bytes()[i] == word.as_bytes()[i] {
            print!("{}", (guess.as_bytes()[i] as char).to_string().green())
        } else if word.contains(guess.as_bytes()[i] as char) {
            print!("{}", (guess.as_bytes()[i] as char).to_string().yellow())
        } else {
            print!("{}", guess.as_bytes()[i] as char)
        }
    }
    println!()
}

fn main() {

    // read list of words
    let mut words: Vec<String> = Vec::new();
    let file = fs::File::open("sgb-words.txt").expect("file cannot be opened");
    for line in io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            words.push(line);
        }
    }

    let word = words.choose(&mut rand::thread_rng())
        .cloned().expect("cannot choose word");

    let mut guesses: Vec<String> = Vec::new();
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        guess = guess.trim().to_lowercase();

        match guess.len() {
            0 => {
                println!("The correct guess was {word}.");
                break;
            }
            5 => {}
            _ => {
                println!("\"{guess}\" is not of length 5.");
                continue;
            }
        };

        if !words.contains(&guess) {
            println!("\"{guess}\" is not a word");
            continue;
        }

        println!("You guessed: \"{guess}\".");

        guesses.push(guess);

        for guess in &guesses {
            print_guess(&guess, &word)
        }
        if &guesses[&guesses.len() - 1] == &word {
            println!("{}", "You win!".green());
            break;
        }
    }
}