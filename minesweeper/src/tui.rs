

extern crate termion;


use std::io::{Write, stdout, stdin};
use std::iter::repeat;
use std::char;

use self::termion::event::Key;
use self::termion::input::TermRead;
use self::termion::raw::IntoRawMode;
use self::termion::cursor::Goto;

use super::minesweeper::{Game, Marker, Tile, Status};

const WIDTH: usize = 16;
const HEIGHT: usize = 16;
const MINE_COUNT: usize = 20;


pub struct App {
    game: Game,
    cursor: (u16, u16),
    stdout: Box<Write>,
}

impl App {
    pub fn new () -> App {
        App {
            game: Game::new(WIDTH, HEIGHT, MINE_COUNT),
            cursor: (0, 0),
            stdout: Box::new(stdout().into_raw_mode().unwrap()),
        }
    }

    pub fn start (&mut self) {
        let stdin = stdin();
        self.clear_all();
        self.render_border();
        self.render_all();
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Char('q') => {
                    self.clear_all();
                    break;
                },

                // movement
                Key::Char('h') => self.move_cursor(-1, 0),
                Key::Char('j') => self.move_cursor(0, 1),
                Key::Char('k') => self.move_cursor(0, -1),
                Key::Char('l') => self.move_cursor(1, 0),

                // place
                Key::Char('f') => self.place(),

                // marker
                Key::Char('x') => self.mark(Marker::Mine),
                Key::Char('u') => self.mark(Marker::Unsure),
                Key::Char('c') => self.mark(Marker::None),

                // reset
                Key::Char('r') => {
                    self.game = Game::new(WIDTH, HEIGHT, MINE_COUNT);
                    self.render_all();
                }
                _ => {},
            }
        }
    }

    fn move_cursor (&mut self, x: i32, y: i32) {
        let nx = self.cursor.0 as i32 + x;
        let ny = self.cursor.1 as i32 + y;
        if 0 <= nx && nx < self.game.width as i32 &&
           0 <= ny && ny < self.game.height as i32 {
            self.cursor = (nx as u16, ny as u16);
            self.render_cursor();
        }
    }

    fn game_position (&self) -> (usize, usize) {
        (self.cursor.0 as usize, self.cursor.1 as usize)
    }

    fn place (&mut self) {
        let p = self.game_position();
        if self.game.marker_at(p) != Marker::None {
            return;
        }
        self.game.reveal(p);
        if self.game.status() == Status::Lose {
            self.game.reveal_all();
        }
        self.render_all();
    }

    fn mark (&mut self, marker: Marker) {
        let p = self.game_position();
        if self.game.status() != Status::Remaining(0) {
            self.game.mark(p, marker);
            self.render_all();
        }
    }

    fn render_all (&mut self) {
        self.render_cells();
        self.render_message();
        self.render_help();
        self.render_cursor();
    }

    fn render_cursor (&mut self) {
        write!(self.stdout, "{}", Goto(self.cursor.0 + 2, self.cursor.1 + 2)).unwrap();
        self.stdout.flush().unwrap();
    }

    fn render_border (&mut self) {
        let style = ('+', '-', '|');
        let horizontal_bar = repeat(style.1).take(WIDTH).collect::<String>();
        let horizontal_space = repeat('.').take(WIDTH).collect::<String>();
        write!(self.stdout, "{}", Goto(1, 1)).unwrap();
        write!(self.stdout, "{}{}{}\n\r", style.0, horizontal_bar, style.0).unwrap();
        for _ in 0..HEIGHT {
            write!(self.stdout, "{}{}{}\n\r", style.2, horizontal_space, style.2).unwrap();
        }
        write!(self.stdout, "{}{}{}\n\r", style.0, horizontal_bar, style.0).unwrap();
        self.stdout.flush().unwrap();
    }

    fn render_cells (&mut self) {
        for j in 0..HEIGHT {
            write!(self.stdout, "{}", Goto(2, j as u16 + 2)).unwrap();
            for i in 0..WIDTH {
                let cell = match self.game.marker_at((i, j)) {
                    Marker::Mine => 'x',
                    Marker::Unsure => '?',
                    Marker::None => match self.game.at((i, j)) {
                        Tile::Revealed(0) => ' ',
                        Tile::Revealed(n) => char::from_digit(n as u32, 10).unwrap(),
                        Tile::Mine => '*',
                        Tile::Unknown => '.',
                    }
                };
                write!(self.stdout, "{}", cell).unwrap();
            }
        }
        self.stdout.flush().unwrap();
    }

    fn render_message (&mut self) {
        let message = match self.game.status() {
            Status::Remaining(0) => String::from("You win :)"),
            Status::Remaining(n) => format!("{} mines remaining", n),
            Status::Lose => String::from("Gameover :("),
        };
        write!(self.stdout, "{}{}{}",
               Goto(0, 3 + HEIGHT as u16),
               termion::clear::CurrentLine,
               message).unwrap();
        self.stdout.flush().unwrap();
    }

    fn render_help (&mut self) {
        write!(self.stdout, "{}HELP: quit(q), movement(hjkl), reveal(f) , marker(cx?)",
               Goto(0, HEIGHT as u16 + 4)).unwrap();
        self.stdout.flush().unwrap();
    }

    fn clear_all (&mut self) {
        write!(self.stdout, "{}{}", termion::clear::All, Goto(1, 1)).unwrap();
    }
}
