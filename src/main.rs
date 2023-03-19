use std::{
	io::{stdin, stdout, Read, Write},
};

mod user;

fn main() {
	let mut user = user::User::default();

	loop {	
		for c in stdin().lock().bytes() {
			let c = c.unwrap() as char;
			if c == '\n' {
				break;
			}
		}		
	
		user.click();
   		print!("{}", user.score());
		stdout().flush().unwrap();
	}
}
