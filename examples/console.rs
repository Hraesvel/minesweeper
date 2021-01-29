use minesweeper::*;
use std::io;
use std::io::{Split, Write};

fn main() -> io::Result<()> {
    let mut buf = String::with_capacity(8);
    let mut stdout = io::stdout();
    let mut session = Session::new(60, 24, 24);
    loop {
        session.print_score();
        session.print_session();
        stdout.lock().write_all(b"$> ")?;
        stdout.flush()?;
        let len = io::stdin().read_line(&mut buf)? - 1;

        if len >= 2 {
            let mut token = buf.trim().split(|c| c == ',' || c == ' ');
            let x = token.next().unwrap().parse().expect("Num");
            let y = token.next().unwrap().parse().expect("Num");
            if !session.check_cord(x, y) {
                session.print_score();
                session.print_answer_fancy();
                println!("You Dead!");
                break;
            }
        }
        buf.clear();
    }

    Ok(())
}
