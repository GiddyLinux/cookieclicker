use std::{
	process,
	io::{stdin, stdout, Write},
};

mod user;

fn main() {
	let mut user = user::User::default();
	let input = &mut String::new();

	loop {	
		input.clear();
		stdin().read_line(input).unwrap();

		match input.trim() {
			"" => {			
				user.click();
   				print!("{}", user.score());
				stdout().flush().unwrap();
			},	
			_ => process::exit(0),
		}
	}
}
