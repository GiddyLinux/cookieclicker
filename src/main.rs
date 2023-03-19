use std::{
	process,
	io::{stdin, stdout, Write},
};

mod user;

fn main() {
	let mut user = user::User::default();
	let mut stdout = stdout();
	loop {
		let mut input = String::new();
		stdin().read_line(&mut input).unwrap();		
	
		match input.trim() {
			"" => {			
				user.click();
   				print!("\r{}", user.score());
				stdout.flush().unwrap();
			},	
			_ => process::exit(0),
		}
	}
}
