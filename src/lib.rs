use rand::{thread_rng, Rng};
use std::{fmt, io};
use std::fmt::Debug;
use std::collections::HashSet;

#[derive(Clone)]
pub enum Tile {
	Mine,
	Hidden(u8),
	Visible(u8)
}

impl Tile {
	fn dfs_valid(&self) -> bool {
		match self {
			Tile::Visible(_) => false,
			Tile::Mine => false,
			_ => true
		}
	}

}

impl Debug for Tile {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		match self {
			Self::Hidden(val) => write!(f, "h{}", val),
			Self::Visible(val) => write!(f, "v{}", val),
			Self::Mine => write!(f, "{}", 'X')
		}
	}
}

trait Board {
	type Item;

	fn new_board(level: usize, x_size: usize, y_size: usize) -> Self::Item;
	fn blank_board(n: usize, m: usize) -> Self::Item;
	fn gen_mine_cords(number: usize, n: usize, m: usize) -> Vec<(usize, usize)> {
		let mut cords = vec![];
		while cords.len() < number {
			let mut c = (thread_rng().gen_range(0, n), thread_rng().gen_range(0, m));
			if !cords.contains(&c) { cords.push(c) }
		}
		cords
	}
}

impl Board for Tile {
	type Item = io::Result<Vec<Vec<Tile>>>;

	fn new_board(level: usize, x_size: usize, y_size: usize) -> Self::Item {
		let mut board = Self::blank_board(x_size, y_size)?;

		for c in Self::gen_mine_cords(level * 3, x_size, y_size) {
			board[c.0][c.1] = Tile::Mine;
		}

		Ok(board)
	}

	fn blank_board(x_size: usize, y_size: usize) -> Self::Item {
		let board: Vec<Vec<Tile>> = (0..y_size)
			.map(|_| {
				(0..x_size).map(|_| Tile::Hidden(0))
				           .collect::<Vec<Tile>>()
			}).collect();
		Ok(board)
	}
}

#[derive(Debug)]
enum State {
	Active,
	Win,
	Lose
}

pub struct MineSweeper {
	session: Option<Session>,
}

pub struct Session {
	des: String,
	state: State,
	board: Vec<Vec<Tile>>,
	score: u32,
}

impl Session {
	pub fn new(level: usize, x_size: usize, y_size: usize) -> Self {
		Self {
			des: "Test".to_string(),
			state: State::Active,
			score: 0,
			board: Tile::new_board(level, x_size, y_size).unwrap()
		}
	}

	pub fn from(board: Vec<Vec<Tile>>) -> Self {
		Self {
			des: "Test".to_string(),
			state: State::Active,
			score: 0,
			board
		}
	}

	pub fn get_board(&self) -> Vec<Vec<Tile>> {
		self.board.clone()
	}

	pub fn check_cord(&mut self, x: usize, y: usize) {
		match self.board[y][x] {
			Tile::Hidden(val) => {
				println!("Safe: {}", &val);
				self.board[y][x] = Tile::Visible(val)
			}
			,
			Tile::Mine => println!("Boom!"),
			Tile::Visible(_) => {}
		}
	}

	pub fn print_session(&self) {
		let mut board = String::new();
		board.push('\n');
		for row in self.board.clone() {
			for col in row {
				match col {
					Tile::Mine => { board.push_str("[ ]") },
					Tile::Hidden(_) => { board.push_str("[ ]") },
					Tile::Visible(val) => { board.push_str(format!("[{}]", val).as_str()) },
				}
//				board.push('');
			}
			board.push('\n');
		}

		print!("{}", board);
	}

	pub fn reveal(&mut self, x: usize, y: usize) {
		match self.board[y][x] {
			Tile::Mine => self.state = State::Lose,
			Tile::Visible(_) => println!("Tile is already visable."),
			Tile::Hidden(_) => Self::dfs(&mut self.board, x, y)
		}
	}

	fn dfs(board: &mut Vec<Vec<Tile>>, x: usize, y: usize) {
		if !board[y][x].dfs_valid() {
			return
		}

		// look up
		if board.get(y + 1).is_some() &&
			board[y].get(x).is_some() &&
			board[y + 1][x].dfs_valid()
			{
			unimplemented!()
		}

		// look down
		if board.get(y - 1).is_some() && board[y].get(x).is_some() {}

		// look right
		if board.get(y).is_some() && board[y].get(x + 1).is_some() {
			unimplemented!()
		}

		// look left
		if board.get(y).is_some() && board[y].get(x - 1).is_some() {
			unimplemented!()
		}
	}
}

impl Debug for Session {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		let mut board = String::new();
		for line in self.board.clone() {
			board.push_str(format!("{:?}\n", line).as_str());
		}
		write!(f, "\n{}", board)
	}
}


#[cfg(test)]
mod tests {
	use super::*;
	use insta::assert_debug_snapshot_matches;

	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}

	#[test]
	fn session_debug_test() {
		let session = Session::new(12, 12, 12);
		session.print_session();
	}

	#[test]
	fn dfs_test() {
		let max = 3;

		let mut matrix: Vec<Vec<Tile>> = (0..4).map(|i| {
			if i == 0 || i == max {
				return vec![Tile::Hidden(1); max + 1];
			} else {
				let mut item = vec![Tile::Hidden(0); max + 1];
				item[0] = Tile::Hidden(1);
				item[max] = Tile::Hidden(1);
				return item;
			}
		}
		)
		                                       .collect();

		let mut sess = Session::from(matrix);

//		dbg!(sess.get_board());

		let expect = vec![
			vec![Tile::Visible(1); 4],
			vec![Tile::Visible(1), Tile::Visible(0), Tile::Visible(0), Tile::Visible(1)],
			vec![Tile::Visible(1), Tile::Visible(0), Tile::Visible(0), Tile::Visible(1)],
			vec![Tile::Visible(1); 4],
		];

//		assert_eq!(format!("{:?}",sess.get_board()), format!("{:?}",expect));

		sess.reveal(0, 0);
	}
}
