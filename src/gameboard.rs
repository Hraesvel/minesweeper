use crate::{Board, Tile};
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct GameBoard(pub Vec<Vec<Tile>>);

impl GameBoard {
    pub fn new(level: usize, x_size: usize, y_size: usize, set_proxy: bool) -> Self {
        let mut board = GameBoard(Tile::blank_board(x_size, y_size).unwrap());

        for dir in Self::gen_mine_coords(level, x_size, y_size) {
            board.0[dir.0][dir.1] = Tile::Mine;
            if set_proxy {
                board.set_neighbor(dir.0 as isize, dir.1 as isize);
            }
        }

        board
    }

    /// generate coordinates as a vector of tuples using `number` for the quantity of mines within a range of `n` and `m`.
    ///
    /// returns `Vec<(usize,usize)>`
    ///
    fn gen_mine_coords(number: usize, n: usize, m: usize) -> Vec<(usize, usize)> {
        let mut cords = vec![];
        while cords.len() < number {
            let c = (thread_rng().gen_range(0, n), thread_rng().gen_range(0, m));
            if !cords.contains(&c) {
                cords.push(c)
            }
        }
        cords
    }

    fn set_neighbor(&mut self, y: isize, x: isize) {
        for dir in &Self::DIRS {
            if y + dir.0 < 0 || x + dir.1 < 0 {
                continue;
            }

            if self.0.get((y + dir.0) as usize).is_some()
                && self.0[(y + dir.0) as usize]
                    .get((x + dir.1) as usize)
                    .is_some()
            {
                if self.0[(y + dir.0) as usize][(x + dir.1) as usize].is_mine() {
                    continue;
                }
                self.0[(y + dir.0) as usize][(x + dir.1) as usize] =
                    match self.0[(y + dir.0) as usize][(x + dir.1) as usize] {
                        Tile::Hidden(val) => Tile::Hidden(val + 1),
                        Tile::Visible(val) => Tile::Visible(val + 1),
                        _ => unreachable!(),
                    };
            }
        }
    }

    fn get_tile(&self, x_cord: usize, y_cord: usize) -> &Tile {
        &self.0[y_cord][x_cord]
    }

    pub fn dfs(&mut self, x: usize, y: usize, is_init: bool) -> u32 {
        //check if tile is invalid
        let mut score = 0;

        if !self.get_tile(x, y).dfs_valid() {
            return 0;
        } else if self.get_tile(x, y).unwrap() > 0 && !is_init {
            self.0[y][x].set_visible();
            return 1;
        }

        // if valid then the tile will be swapped to visible.
        self.0[y][x].set_visible();

        // look up
        if y as isize - 1 >= 0 && self.0.get(y - 1).is_some() && self.0[y].get(x).is_some() {
            score += self.dfs(x, y - 1, false);
        }

        // look down
        if self.0.get(y + 1).is_some() && self.0[y].get(x).is_some() {
            score += self.dfs(x, y + 1, false);
        }

        // look right
        if self.0.get(y).is_some() && self.0[y].get(x + 1).is_some() {
            score += self.dfs(x + 1, y, false);
        }

        // look left
        if x as isize - 1 >= 0 && self.0.get(y).is_some() && self.0[y].get(x - 1).is_some() {
            score += self.dfs(x - 1, y, false);
        }

        score
    }

    const DIRS: [(isize, isize); 8] = [
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
    ];
}
