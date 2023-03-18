use std::{
	process,
	io::{stdout, Write},
};

use termion::{
	clear, 
	cursor,
};

mod user;

fn main() {
	let mut stdout = stdout();

	let mut user = user::User::default();
	loop {
		let mut input = String::new();
		std::io::stdin().read_line(&mut input).unwrap();

		match input.trim() {
			"" => {
				write!(stdout, "cookies").unwrap();			
				user.click();
				write!(stdout, "{}{}", cursor::Up(1), clear::CurrentLine).unwrap();
				write!(stdout, "\r{}", user.score()).unwrap();
				stdout.flush().unwrap();
			},	
			_ => process::exit(0),
		}
	}
}
