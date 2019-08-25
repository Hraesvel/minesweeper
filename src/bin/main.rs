use minesweeper::*;
use std::io;
use std::io::{Read, Write, BufReader};

fn main() -> io::Result<()>{

	let mut buf = String::with_capacity(8);
	let mut stdout = io::stdout();
	let mut session = Session::new(12, 12, 12);
	loop {
		session.print_session();
		stdout.lock().write_all(b"$> ")?;
		stdout.flush()?;
		let len = io::stdin().read_line(&mut buf)?;

		let mut token = buf.trim().split(|c| c == ',' || c == ' ');
		let x = token.next().unwrap().parse().expect("Num");
		let y = token.next().unwrap().parse().expect("Num");
		if len  > 2 {
			session.check_cord(x,y);
//			stdout
//				.lock()
//				.write(buf.as_bytes())?;

		}
		buf.clear();


	}
	Ok(())
}
