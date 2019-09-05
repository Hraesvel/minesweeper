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
	/// swaps a Tile Hidden to Visible
	fn set_visible(&mut self) {
		if let Tile::Hidden(val) = self {
			*self = Tile::Visible(*val);
		}
	}
}

impl Debug for Tile {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		match self {
			Self::Hidden(val) => write!(f, "h{}", val),
			Self::Visible(val) => write!(f, "v{}", val),
			Self::Mine => write!(f, "{}", "💣"),
		}
	}
}

trait Board {
	type Item;

	/// create a new game board by X and Y size.
	fn new_board(level: usize, x_size: usize, y_size: usize) -> Self::Item;
	/// create a new game blank board by X and Y size.
	fn blank_board(n: usize, m: usize) -> Self::Item;

	/// generate coordinates as a vector of tuples using `number` for the quantity of mines within a range of `n` and `m`.
	///
	/// returns `Vec<(usize,usize)>`
	///
	fn gen_mine_coords(number: usize, n: usize, m: usize) -> Vec<(usize, usize)> {
		let mut cords = vec![];
		while cords.len() < number {
			let mut c = (thread_rng().gen_range(0, n), thread_rng().gen_range(0, m));
			if !cords.contains(&c) { cords.push(c) }
		}
		cords
	}
	/// converts a string representation into a board of associated type
	fn from_string<S: Into<String>>(input: S, hidden: bool) -> Self::Item;
}

impl Board for Tile {
	type Item = io::Result<Vec<Vec<Tile>>>;

	/// create a new game board by X and Y size with mine randomly place.
	fn new_board(level: usize, x_size: usize, y_size: usize) -> Self::Item {
		let mut board = Self::blank_board(x_size, y_size)?;

		for c in Self::gen_mine_coords(level * 3, x_size, y_size) {
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

	/// Converts a string representation into a board of Tile types.
	fn from_string<S: Into<String>>(input: S, hidden: bool) -> Self::Item {
		let board: Vec<Vec<Tile>> = input.into().split_whitespace()
		                                 .into_iter()
		                                 .map(|x| {
			                                 x.split(',')
			                                  .into_iter()
			                                  .map(|x| {
				                                  if let Ok(value) = x.parse::<u8>() {
					                                  if hidden {
						                                  return Tile::Hidden(value)
					                                  }
					                                  return Tile::Visible(value)
				                                  }
				                                  Tile::Mine
			                                  })
			                                  .collect::<Vec<Tile>>()
		                                 }
		                                 )
		                                 .collect();
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

	pub fn print_session(&self) -> String {
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

		print!("{}", &board);

		board
	}

	pub fn print_answer(&self) -> String {
		let mut board = String::new();
		board.push('\n');
		for row in self.board.clone() {
			for col in row {
				match col {
					Tile::Mine => { board.push_str("[💣]") },
					Tile::Hidden(val) => { board.push_str(format!("[{}]", val).as_str()) },
					Tile::Visible(val) => { board.push_str(format!("[{}]", val).as_str()) },
				}
//				board.push('');
			}
			board.push('\n');
		}

		print!("{}", &board);
		board
	}

	pub fn reveal(&mut self, x: usize, y: usize) {
		match self.board[y][x] {
			Tile::Mine => self.state = State::Lose,
			Tile::Visible(_) => println!("Tile is already visable."),
			Tile::Hidden(_) => Self::dfs(&mut self.board, x, y)
		}
	}

	fn dfs(board: &mut Vec<Vec<Tile>>, x: usize, y: usize) {
		//check if tile is invalid
		if !board[y][x].dfs_valid() {
			return
		}

		// if valid then the tile will be swapped to visible.
		board[y][x].set_visible();

		// look up
		if y as isize - 1 >= 0 &&
			board.get(y - 1).is_some() &&
			board[y].get(x).is_some()
		{
			Self::dfs(board, x, y - 1);
		}

		// look down
		if board.get(y + 1).is_some() && board[y].get(x).is_some() {
			Self::dfs(board, x, y + 1);
		}

		// look right
		if board.get(y).is_some() && board[y].get(x + 1).is_some() {
			Self::dfs(board, x + 1, y);
		}

		// look left
		if x as isize - 1 >= 0 && board.get(y).is_some() && board[y].get(x - 1).is_some() {
			Self::dfs(board, x - 1, y);
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
	fn test_from_string() {
		let m: String =
			"
    1,1,1,1
    1,0,X,1
    1,0,0,1
    1,1,1,1
    ".to_string();

		let matrix = Tile::from_string(m, true).unwrap();
		let session = Session::from(matrix);

		session.print_session();
		session.print_answer();

		let expect = vec![
			vec![Tile::Hidden(1); 4],
			vec![Tile::Hidden(1), Tile::Hidden(0), Tile::Mine, Tile::Hidden(1)],
			vec![Tile::Hidden(1), Tile::Hidden(0), Tile::Hidden(0), Tile::Hidden(1)],
			vec![Tile::Hidden(1); 4],
		];

		assert_eq!(format!("{:?}", session.get_board()), format!("{:?}", expect));
	}

	#[test]
	// TODO: create a test that uses assert macro in some way.
	fn session_debug_test() {
		let session = Session::new(12, 12, 12);
		session.print_session();
	}

	#[test]
	fn dfs_test() {
		let max = 4;

		let mut matrix: Vec<Vec<Tile>> = (0..max).map(|i| {
			if i == 0 || i == max - 1 {
				return vec![Tile::Hidden(1); max];
			} else {
				let mut item = vec![Tile::Hidden(0); max];
				item[0] = Tile::Hidden(1);
				item[max - 1] = Tile::Hidden(1);
				return item;
			}
		}
		)
		                                         .collect();

		let mut sess = Session::from(matrix);

		let expect = vec![
			vec![Tile::Visible(1); 4],
			vec![Tile::Visible(1), Tile::Visible(0), Tile::Visible(0), Tile::Visible(1)],
			vec![Tile::Visible(1), Tile::Visible(0), Tile::Visible(0), Tile::Visible(1)],
			vec![Tile::Visible(1); 4],
		];

		sess.reveal(0, 0);

		assert_eq!(format!("{:?}", sess.get_board()), format!("{:?}", expect));
	}

	#[test]
	fn dfs_test_with_mine() {
		let max = 4;

		let mut matrix: Vec<Vec<Tile>> = (0..max).map(|i| {
			if i == 0 || i == max - 1 {
				return vec![Tile::Hidden(1); max];
			} else {
				let mut item = vec![Tile::Hidden(0); max];
				item[0] = Tile::Hidden(1);
				item[max - 1] = Tile::Hidden(1);
				return item;
			}
		}
		)
		                                         .collect();

		let mut sess = Session::from(matrix);
		sess.board[1][2] = Tile::Mine;

		let expect = vec![
			vec![Tile::Visible(1); 4],
			vec![Tile::Visible(1), Tile::Visible(0), Tile::Mine, Tile::Visible(1)],
			vec![Tile::Visible(1), Tile::Visible(0), Tile::Visible(0), Tile::Visible(1)],
			vec![Tile::Visible(1); 4],
		];

		sess.reveal(0, 0);
		dbg!(&sess.board);

		assert_eq!(format!("{:?}", sess.get_board()), format!("{:?}", expect));
	}

	#[test]
	fn dfs_test_with_island() {
		let matrix =
			"
1,1,1,1,1,1,1
1,0,0,0,0,0,1
1,0,X,X,X,0,1
1,0,X,2,X,0,1
1,0,X,X,X,0,1
1,0,0,0,0,0,1
1,1,1,1,1,1,1
";

		let mut sess = Session::from(Tile::from_string(matrix, true).unwrap());
		let mut expect = Tile::from_string(matrix, false).unwrap();
		expect[3][3] = Tile::Hidden(2);

		sess.reveal(0, 0);
		sess.print_session();

		assert_eq!(format!("{:?}", sess.get_board()), format!("{:?}", expect));
	}

	#[test]
	fn test_show_answer() {
		let max = 4;

		let mut matrix: Vec<Vec<Tile>> = (0..max).map(|i| {
			if i == 0 || i == max - 1 {
				return vec![Tile::Hidden(1); max];
			} else {
				let mut item = vec![Tile::Hidden(0); max];
				item[0] = Tile::Hidden(1);
				item[max - 1] = Tile::Hidden(1);
				return item;
			}
		}
		)
		                                         .collect();

		let mut sess = Session::from(matrix);
		sess.board[1][2] = Tile::Mine;

		assert_ne!(sess.print_session(), sess.print_answer());
	}


	#[test]
	fn make_visible() {
		let mut tile = Tile::Hidden(2);
		tile.set_visible();

		dbg!(tile);
	}
}
