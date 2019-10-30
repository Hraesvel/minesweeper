use rand::{thread_rng, Rng};

use std::fmt::Debug;
use std::{fmt, io};

pub use crate::board::Board;
pub use crate::session::Session;
pub use crate::tile::Tile;
pub use crate::gameboard::GameBoard;

mod board;
mod session;
mod tile;
mod gameboard;



#[derive(Debug)]
enum State {
	Active,
	Win,
	Lose,
}

pub struct MineSweeper {
	session: Option<Session>,
}

impl MineSweeper {
	pub fn new() -> Self {
		Self {
			session: None
		}
	}

	pub fn start_session(&mut self, level: u8) {
//		self.session = Session::new();
	}
}
