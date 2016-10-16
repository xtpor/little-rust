
extern crate rand;

use self::rand::Rng;



pub type Point = (usize, usize);

fn get<T: Copy> (grid: &Vec<Vec<T>>, position: Point) -> T {
    grid[position.0][position.1]
}

fn set<T> (grid: &mut Vec<Vec<T>>, position: Point, value: T) {
    grid[position.0][position.1] = value
}

fn is_adjacent (a: Point, b: Point) -> bool {
    let diff_x = (a.0 as i32 - b.0 as i32).abs();
    let diff_y = (a.1 as i32 - b.1 as i32).abs();
    diff_x <= 1 && diff_y <= 1 && diff_x + diff_y > 0
}

fn new_vec2<T: Copy> (width: usize, height: usize, value: T) -> Vec<Vec<T>> {
    vec![vec![value; height]; width]
}

#[derive(Copy, Clone, PartialEq)]
pub enum Tile {
    Revealed(usize),
    Mine,
    Unknown,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Status {
    Remaining(usize),
    Lose,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Marker {
    Mine,
    Unsure,
    None
}

pub struct Game {
    pub width: usize,
    pub height: usize,
    pub mine_count: usize,
    tiles: Vec<Vec<Tile>>,
    markers: Vec<Vec<Marker>>,
    mines: Vec<Vec<bool>>,
}

impl Game {
    pub fn new (width: usize, height: usize, mine_count: usize) -> Game {
        let mut game = Game {
            width: width,
            height: height,
            mine_count: mine_count,
            tiles: new_vec2(width, height, Tile::Unknown),
            markers: new_vec2(width, height, Marker::None),
            mines: new_vec2(width, height, false),
        };
        game.generate_mines();
        game
    }

    pub fn status (&mut self) -> Status {
        let mines_count = self.walk().filter(|&p| self.at(p) == Tile::Mine).count();
        if mines_count > 0 {
            Status::Lose
        } else {
            let unknown_count = self.walk().filter(|&p| self.is_unknown(p)).count();
            Status::Remaining(unknown_count - self.mine_count)
        }
    }

    pub fn mark (&mut self, pos: Point, marker: Marker) {
        if self.is_unknown(pos) {
            set(&mut self.markers, pos, marker);
        }
    }

    pub fn at (&self, pos: Point) -> Tile {
        get(&self.tiles, pos)
    }

    pub fn marker_at (&self, pos: Point) -> Marker {
        get(&self.markers, pos)
    }

    fn walk (&self) -> Box<Iterator<Item=Point>> {
        let (w, h) = (self.width, self.height);
        Box::new((0..w).flat_map(move |i| (0..h).map(move |j| (i, j))))
    }

    fn has_mine (&self, pos: Point) -> bool {
        get(&self.mines, pos)
    }

    fn is_unknown (&self, pos: Point) -> bool {
        self.at(pos) == Tile::Unknown
    }

    fn generate_mines (&mut self) {
        let mut count = self.mine_count;
        let mut rng = rand::thread_rng();
        while count > 0 {
            let position = (rng.gen_range(0, self.width), rng.gen_range(0, self.height));
            if !self.has_mine(position) {
                set(&mut self.mines, position, true);
                count -= 1;
            }
        }
    }

    fn adjacent_mine_count (&self, pos: Point) -> usize {
        self.adjacent(pos).filter(|&p| self.has_mine(p)).count()
    }

    fn adjacent (&self, pos: Point) -> Box<Iterator<Item=Point>> {
        Box::new(self.walk().filter(move |&p| is_adjacent(p, pos)))
    }

    pub fn reveal_all (&mut self) {
        for p in self.walk() {
            self.reveal(p);
        }
    }

    pub fn reveal (&mut self, pos: Point) {
        self.reveal_single(pos);
        if self.at(pos) == Tile::Revealed(0) {
            for next in self.adjacent(pos) {
                if self.is_unknown(next) {
                    self.reveal(next);
                }
            }
        }
    }

    fn reveal_single (&mut self, pos: Point) {
        if self.is_unknown(pos) {
            self.mark(pos, Marker::None);
            let tile = if self.has_mine(pos) {
                Tile::Mine
            } else {
                Tile::Revealed(self.adjacent_mine_count(pos))
            };
            set(&mut self.tiles, pos, tile);
        }
    }

}
