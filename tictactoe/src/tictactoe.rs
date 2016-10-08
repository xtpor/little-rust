
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    X, O,
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tile {
    Piece(Player), Empty,
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Status {
    Move(Player), Win(Player), Draw
}


#[derive(Debug)]
pub struct Game {
    pub status: Status,
    grid: [Tile; 9],
}


impl Game {

    pub fn new () -> Game {
        Game {
            grid: [Tile::Empty; 9],
            status: Status::Move(Player::O),
        }
    }

    pub fn tile_at (&self, x: i32, y: i32) -> Option<Tile> {
        let p = (x * 3 + y) as usize;
        match p {
            0 ... 9 => Some(self.grid[p]),
            _       => None,
        }
    }

    pub fn put (&mut self, x: i32, y: i32) -> Result<(), &'static str> {
        match self.tile_at(x, y) {
            Some(tile) => match self.status {
                Status::Move(player) => match tile {
                    Tile::Empty => {
                        let p = (x * 3 + y) as usize;
                        self.grid[p] = Tile::Piece(player);
                        self.status = self.next_status();
                        Ok(())
                    },
                    _ => Err("Non-empty position"),
                },
                _ => Err("The game is alreadly ended"),
            },
            None => Err("Invalid position"),
        }
    }

    pub fn next_status (&self) -> Status {
        let patterns = [[0, 1, 2],
                        [3, 4, 5],
                        [6, 7, 8],
                        [0, 3, 6],
                        [1, 4, 7],
                        [2, 5, 8],
                        [0, 4, 8],
                        [2, 4, 6]];

        for &pattern in patterns.iter() {
            if let Tile::Piece(player) = self.match_tiles(pattern) {
                return Status::Win(player);
            }
        }

        if self.is_full() {
            return Status::Draw;
        }

        match self.status {
            Status::Move(Player::O) => Status::Move(Player::X),
            Status::Move(Player::X) => Status::Move(Player::O),
            _ => unimplemented!(),
        }
    }

    fn match_tiles (&self, pattern: [usize; 3]) -> Tile {
        for &player in [Player::O, Player::X].iter() {
            let tile = Tile::Piece(player);
            if pattern.iter().all(|&p| self.grid[p] == tile) {
                return tile;
            }
        }
        Tile::Empty
    }

    fn is_full (&self) -> bool {
        self.grid.iter().all(|&t| t != Tile::Empty)
    }

}
