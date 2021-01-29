use crate::{GameBoard, State, Tile};
use colored::Colorize;
use std::fmt::Debug;
use std::{fmt, io};

pub struct Session {
    des: String,
    state: State,
    board: GameBoard,
    score: u32,
}

impl Session {
    pub fn new(level: usize, x_size: usize, y_size: usize) -> Self {
        Self {
            des: "Test".to_string(),
            state: State::Active,
            score: 0,
            board: GameBoard::new(level, x_size, y_size, true),
        }
    }

    pub fn from(board: Vec<Vec<Tile>>) -> Self {
        Self {
            des: "Test".to_string(),
            state: State::Active,
            score: 0,
            board: GameBoard(board),
        }
    }

    pub fn mut_board(&mut self) -> &mut Vec<Vec<Tile>> {
        &mut self.board.0
    }

    pub fn get_board(&self) -> Vec<Vec<Tile>> {
        self.board.0.clone()
    }

    pub fn check_cord(&mut self, x: usize, y: usize) -> bool {
        match self.board.0[y][x] {
            Tile::Hidden(val) => {
                println!("Safe: {}", &val);
                self.reveal(x, y);
                self.board.0[y][x] = Tile::Visible(val)
            }
            Tile::Mine => {
                println!("Boom!");
                return false;
            }
            Tile::Visible(_) => println!("Already Visible"),
        }
        true
    }

    pub fn print_score(&self) {
        println!("Score: {}", self.score);
    }

    pub fn print_session(&self) -> String {
        let mut board = String::new();
        board.push('\n');
        for row in self.board.0.clone() {
            for col in row {
                match col {
                    Tile::Mine => board.push_str("[ ]"),
                    Tile::Hidden(_) => board.push_str("[ ]"),
                    Tile::Visible(val) => board.push_str(format!("[{}]", val).as_str()),
                }
            }
            board.push('\n');
        }

        print!("{}", &board);

        board
    }

    pub fn print_answer(&self) -> String {
        let mut board = String::new();
        board.push('\n');
        for row in self.board.0.clone() {
            for col in row {
                match col {
                    Tile::Mine => board.push_str("[B]"),
                    Tile::Hidden(val) => board.push_str(format!("[{}]", val).as_str()),
                    Tile::Visible(val) => board.push_str(format!("[{}]", val).as_str()),
                }
                //				board.push('');
            }
            board.push('\n');
        }

        print!("{}", &board);
        board
    }

    pub fn print_answer_fancy(&self) {
        let mut board = String::new();
        board.push('\n');
        for row in self.board.0.clone() {
            for col in row {
                match col {
                    Tile::Mine => print!("[{}]", "B".red()),
                    Tile::Hidden(val) => print!("[{}]", val),
                    Tile::Visible(val) => print!("[{}]", val.to_string().green()),
                }
            }
            println!();
        }
    }

    pub fn reveal(&mut self, x: usize, y: usize) {
        match self.board.0[y][x] {
            Tile::Mine => self.state = State::Lose,
            Tile::Visible(_) => println!("Tile is already visable."),
            Tile::Hidden(_) => self.score += self.board.dfs(x, y, true),
        }
    }
}

impl Debug for Session {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut board = String::new();
        for line in self.board.0.clone() {
            board.push_str(format!("{:?}\n", line).as_str());
        }
        write!(f, "\n{}", board)
    }
}
