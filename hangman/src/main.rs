
extern crate rand;

use rand::Rng;
use std::io;
use std::io::prelude::*;

enum GuessResult {
    Win,
    Continue,
    Repeated,
    NotPrecent,
}

struct Game {
    answer: String,
    guesses: Vec<char>,
    missed: i32,
}

impl Game {

    fn new (answer: &'static str) -> Game {
        Game {
            answer: answer.to_string(),
            guesses: vec![],
            missed: 0,
        }
    }

    fn guess (&mut self, guess: char) -> GuessResult {
        if self.guesses.contains(&guess) {
            GuessResult::Repeated
        } else if self.answer.chars().any(|c| c == guess) {
            self.guesses.push(guess);
            if self.has_won() {
                GuessResult::Win
            } else {
                GuessResult::Continue
            }
        } else {
            self.missed += 1;
            GuessResult::NotPrecent
        }
    }

    fn show (&self, result: &mut String) {
        result.clear();
        for c in self.answer.chars() {
            if self.guesses.contains(&c) {
                result.push(c);
            } else {
                result.push('*');
            }
        }
    }

    fn has_won (&self) -> bool {
        let mut state = String::new();
        self.show(&mut state);
        return state.chars().all(|c| c != '*');
    }

}

fn main () {
    let mut rng = rand::thread_rng();
    let word_list = include_str!("words.txt");
    let words: Vec<_> = word_list.split('\n').collect();

    let mut display = String::new();
    let mut input = String::new();
    let mut game = Game::new(rng.choose(&words).unwrap());
    loop {
        game.show(&mut display);
        print!("(Guess) Enter a letter in word {} > ", display);
        io::stdout().flush().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        if input.len() > 0 {
            let c = input.chars().nth(0).unwrap();
            match game.guess(c) {
                GuessResult::Repeated => println!("     {} is alreadly in the word", c),
                GuessResult::NotPrecent => println!("     {} is not in the word", c),
                GuessResult::Continue => (),
                GuessResult::Win => break,
            }
        } else {
            println!("     unknown input");
        }
    }
    println!("You win!");
    println!("The answer is {}, you have missed {} time(s)", game.answer, game.missed);
}
