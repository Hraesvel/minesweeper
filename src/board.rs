use rand::{thread_rng, Rng};

pub trait Board {
	type Item;

	/// create a new game board by X and Y size.
	fn new_board(level: usize, x_size: usize, y_size: usize) -> Self::Item;
	/// create a new game blank board by X and Y size.
	fn blank_board(n: usize, m: usize) -> Self::Item;

	#[deprecated]
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

	/// converts a string representation into a board of associated type
	fn from_string<S: Into<String>>(input: S, hidden: bool) -> Self::Item;

	#[deprecated]
	fn get_neighbor(board: Self::Item, y: usize, x: usize) -> Self::Item;

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
