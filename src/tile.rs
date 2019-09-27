use crate::Board;
use std::fmt::Debug;
use std::{fmt, io};

#[derive(Clone)]
pub enum Tile {
	Mine,
	Hidden(u8),
	Visible(u8),
}

impl Tile {
	pub fn dfs_valid(&self) -> bool {
		match self {
			Tile::Visible(_) => false,
			Tile::Mine => false,
			_ => true,
		}
	}

	pub fn unwrap(&self) -> u8 {
		if let Self::Hidden(val) = self {
			return *val;
		}
		return 0;
	}

	pub fn is_mine(&self) -> bool {
		if let Tile::Mine = self {
			return true;
		}
		false
	}
	/// swaps a Tile Hidden to Visible
	pub fn set_visible(&mut self) {
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

impl Board for Tile {
	type Item = io::Result<Vec<Vec<Tile>>>;

	/// create a new game board by X and Y size with mine randomly place.
	fn new_board(level: usize, x_size: usize, y_size: usize) -> Self::Item {
		let mut board = Self::blank_board(x_size, y_size)?;

		let mines = Self::gen_mine_coords(level, x_size, y_size);

		for c in &mines {
			board[c.0][c.1] = Tile::Mine;
		}

		Ok(board)
	}

	fn blank_board(x_size: usize, y_size: usize) -> Self::Item {
		let board: Vec<Vec<Tile>> = (0..y_size)
			.map(|_| (0..x_size).map(|_| Tile::Hidden(0)).collect::<Vec<Tile>>())
			.collect();
		Ok(board)
	}

	/// Converts a string representation into a board of Tile types.
	fn from_string<S: Into<String>>(input: S, hidden: bool) -> Self::Item {
		let board: Vec<Vec<Tile>> = input
			.into()
			.split_whitespace()
			.into_iter()
			.map(|x| {
				x.split(',')
				 .into_iter()
				 .map(|x| {
					 if let Ok(value) = x.parse::<u8>() {
						 if hidden {
							 return Tile::Hidden(value);
						 }
						 return Tile::Visible(value);
					 }
					 Tile::Mine
				 })
				 .collect::<Vec<Tile>>()
			})
			.collect();
		Ok(board)
	}

	fn get_neighbor(board: Self::Item, y: usize, x: usize) -> Self::Item {
		if board.is_err() {
			panic!("Error: {:?}", board);
		}
		let mut board = board.unwrap();

		for dir in &Self::DIRS {
			if y as isize + dir.0 < 0 || x as isize + dir.1 < 0 {
				continue;
			}

			if board.get((y as isize + dir.0) as usize).is_some()
				&& board[(y as isize + dir.0) as usize]
				.get((x as isize + dir.1) as usize)
				.is_some()
			{
				if board[(y as isize + dir.0) as usize][(x as isize + dir.1) as usize].is_mine() {
					continue;
				}
				board[(y as isize + dir.0) as usize][(x as isize + dir.1) as usize] =
					match board[(y as isize + dir.0) as usize][(x as isize + dir.1) as usize] {
						Tile::Hidden(val) => Tile::Hidden(val + 1),
						Tile::Visible(val) => Tile::Visible(val + 1),
						_ => unreachable!(),
					};
			}
		}

		Ok(board)
	}
}
