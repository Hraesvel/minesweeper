use rand::{thread_rng, Rng};

pub trait Board {
	type Item;

	/// create a new game board by X and Y size.
	fn new_board(level: usize, x_size: usize, y_size: usize, mines: Vec<(usize, usize)>) -> Self::Item;
	/// create a new game blank board by X and Y size.
	fn blank_board(n: usize, m: usize) -> Self::Item;

	/// converts a string representation into a board of associated type
	fn from_string<S: Into<String>>(input: S, hidden: bool) -> Self::Item;

	// 8 directions
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
