use std::{
	process,
	io::{stdout, Write},
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
				user.click();
				print!("\r");
   				print!("{}", user.score());
				stdout.flush().unwrap();
			},	
			_ => process::exit(0),
		}
	}
}
