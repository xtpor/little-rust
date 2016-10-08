
mod tictactoe;

use std::io;
use std::io::prelude::*;

use tictactoe::Game;
use tictactoe::Tile;
use tictactoe::Player;
use tictactoe::Status;


impl Player {

    fn to_string (self) -> &'static str {
        match self {
            Player::X => "X",
            Player::O => "O",
        }
    }

}

impl Game {

    fn display_board (&self) {
        println!("*****");
        for y in 0..3 {
            print!("*");
            for x in 0..3 {
                let tile = self.tile_at(x, y).unwrap();
                let repr = match tile {
                    Tile::Empty => " ",
                    Tile::Piece(player) => player.to_string(),
                };
                print!("{}", repr);
            }
            println!("*");
        }
        println!("*****");
    }

    fn display_status(&self) {
        match self.status {
            Status::Move(player) => println!("Player {} move.", player.to_string()),
            Status::Win(player) => println!("Player {} win.", player.to_string()),
            Status::Draw => println!("Draw game."),
        }
    }

}

fn main() {
    let mut game = Game::new();
    loop {
        game.display_status();
        game.display_board();

        println!("input position x: ");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Unable to readline.");
        let x: i32 = match buffer.trim().parse() {
            Ok(number) => number,
            Err(_)     => continue,
        };

        println!("input position y: ");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Unable to readline.");
        let y: i32 = match buffer.trim().parse() {
            Ok(number) => number,
            Err(_)     => continue,
        };

        match game.put(x, y) {
            Ok(_) => (),
            Err(_) => {
                println!("Invalid action.");
                continue;
            },
        };

        match game.status {
            Status::Move(_) => (),
            _               => {
                game.display_status();
                break;
            },
        }
    }
}
