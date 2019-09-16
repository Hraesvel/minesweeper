#[cfg(test)]
mod tests {
	use minesweeper::*;


	#[test]
	// TODO: create a test that uses assert macro in some way.
	fn session_print_outs_test() {
		let mut session = Session::new(15, 12, 12);
		dbg!(&session.board());
		session.print_session();
		session.print_answer();
	}

	#[test]
	fn test_from_string() {
		let m: String = "
    1,1,1,1
    1,0,X,1
    1,0,0,1
    1,1,1,1
    "
			.to_string();

		let matrix = Tile::from_string(m, true).unwrap();
		let session = Session::from(matrix);

		session.print_session();
		session.print_answer();

		let expect = vec![
			vec![Tile::Hidden(1); 4],
			vec![
				Tile::Hidden(1),
				Tile::Hidden(0),
				Tile::Mine,
				Tile::Hidden(1),
			],
			vec![
				Tile::Hidden(1),
				Tile::Hidden(0),
				Tile::Hidden(0),
				Tile::Hidden(1),
			],
			vec![Tile::Hidden(1); 4],
		];

		assert_eq!(
			format!("{:?}", session.get_board()),
			format!("{:?}", expect)
		);
	}

	#[test]
	fn dfs_test() {
		let max = 4;

		let matrix: Vec<Vec<Tile>> = (0..max)
			.map(|i| {
				if i == 0 || i == max - 1 {
					return vec![Tile::Hidden(1); max];
				} else {
					let mut item = vec![Tile::Hidden(0); max];
					item[0] = Tile::Hidden(1);
					item[max - 1] = Tile::Hidden(1);
					return item;
				}
			})
			.collect();

		let mut sess = Session::from(matrix);

		let expect = vec![
			vec![Tile::Visible(1); 4],
			vec![
				Tile::Visible(1),
				Tile::Visible(0),
				Tile::Visible(0),
				Tile::Visible(1),
			],
			vec![
				Tile::Visible(1),
				Tile::Visible(0),
				Tile::Visible(0),
				Tile::Visible(1),
			],
			vec![Tile::Visible(1); 4],
		];

		sess.reveal(0, 0);

		assert_eq!(format!("{:?}", sess.get_board()), format!("{:?}", expect));
	}

	#[test]
	fn dfs_test_with_mine() {
		let max = 4;

		let matrix: Vec<Vec<Tile>> = (0..max)
			.map(|i| {
				if i == 0 || i == max - 1 {
					return vec![Tile::Hidden(1); max];
				} else {
					let mut item = vec![Tile::Hidden(0); max];
					item[0] = Tile::Hidden(1);
					item[max - 1] = Tile::Hidden(1);
					return item;
				}
			})
			.collect();

		let mut sess = Session::from(matrix);
		sess.board()[1][2] = Tile::Mine;

		let expect = vec![
			vec![Tile::Visible(1); 4],
			vec![
				Tile::Visible(1),
				Tile::Visible(0),
				Tile::Mine,
				Tile::Visible(1),
			],
			vec![
				Tile::Visible(1),
				Tile::Visible(0),
				Tile::Visible(0),
				Tile::Visible(1),
			],
			vec![Tile::Visible(1); 4],
		];

		sess.print_session();
		sess.reveal(0, 0);
		sess.print_session();
		//        dbg!(&sess.board);

		assert_eq!(format!("{:?}", sess.get_board()), format!("{:?}", expect));
	}

	#[test]
	fn dfs_test_with_island() {
		let matrix = "
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

		let matrix: Vec<Vec<Tile>> = (0..max)
			.map(|i| {
				if i == 0 || i == max - 1 {
					return vec![Tile::Hidden(1); max];
				} else {
					let mut item = vec![Tile::Hidden(0); max];
					item[0] = Tile::Hidden(1);
					item[max - 1] = Tile::Hidden(1);
					return item;
				}
			})
			.collect();

		let mut sess = Session::from(matrix);
		sess.board()[1][2] = Tile::Mine;

		assert_ne!(sess.print_session(), sess.print_answer());
	}

	#[test]
	fn make_visible() {
		let mut tile = Tile::Hidden(2);
		tile.set_visible();

		dbg!(tile);
	}
}
