
extern crate rand;

use std::char;
use std::io;
use std::io::prelude::*;


struct Digits {
    digits: [u32; 4]
}

impl Digits {
    fn new (numbers: &str) -> Option<Digits> {
        let mut digit_set = [false; 10];
        let mut guess = [0; 4];
        if numbers.len() != 4 {
            return None;
        }
        for (i, c) in numbers.chars().enumerate() {
            match c.to_digit(10) {
                Some(digit) => {
                    if digit_set[digit as usize] {
                        return None;
                    }
                    digit_set[digit as usize] = true;
                    guess[i] = digit;
                },
                None => return None,
            }
        }
        Some(Digits {digits: guess})
    }

    fn generate () -> Digits {
        let mut gen = rand::thread_rng();
        let random_digits = rand::sample(&mut gen, 0..10, 4)
            .iter()
            .map(|&d| char::from_digit(d as u32, 10).unwrap())
            .collect::<String>();
        Digits::new(&random_digits).unwrap()
    }

    fn check (&self, guess: &Digits) -> (u32, u32) {
        let mut correct_count = 0;
        let mut diff_count = 0;
        for (i, &self_digit) in self.digits.iter().enumerate() {
            if self_digit == guess.digits[i] {
                correct_count += 1;
            } else if guess.digits.iter().any(|&d| d == self_digit) {
                diff_count += 1;
            }
        }
        (correct_count, diff_count)
    }

    fn check_correct (&self, guess: &Digits) -> bool {
        let (correct_count, _) = self.check(guess);
        correct_count == 4
    }
}

fn main () {
    let answer = Digits::generate();
    let mut buffer = String::new();
    let mut times = 0;

    println!("This is the game of bulls and cows");
    println!("Enter your guess below, e.g. 1234");

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        buffer.clear();
        io::stdin().read_line(&mut buffer).unwrap();
        buffer.pop(); // remove trailing newline

        match Digits::new(&buffer) {
            Some(digits) => {
                times += 1;
                if answer.check_correct(&digits) {
                    break;
                } else {
                    let (a, b) = answer.check(&digits);
                    println!("{}A{}B", a, b);
                }
            },
            None => println!("Error: Invalid format"),
        }
    }

    println!("Correct! You guess {} time(s).", times);
}

#[test]
fn digits_format () {
    // valid case
    assert!(Digits::new("1234").is_some());
    assert!(Digits::new("5678").is_some());
    // invalid case
    assert!(Digits::new("234").is_none()); // length < 4
    assert!(Digits::new("12345").is_none()); // length > 4
    assert!(Digits::new("234a").is_none()); // contains not only digits
    assert!(Digits::new("1341").is_none()); // contains repeated digits
}

#[test]
fn digits_guessing () {
    let answer = Digits::new("1357").unwrap();

    let guess1 = Digits::new("1234").unwrap();
    let guess2 = Digits::new("7135").unwrap();
    let guess3 = Digits::new("1357").unwrap();
    assert_eq!(answer.check(&guess1), (1, 1));
    assert_eq!(answer.check(&guess2), (0, 4));
    assert_eq!(answer.check(&guess3), (4, 0));
}
