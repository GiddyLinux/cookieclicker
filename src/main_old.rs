use std::process;
mod user;

fn main() {
	let mut user = user::User::default();
	loop {
		let mut input = String::new();
		std::io::stdin().read_line(&mut input).unwrap();

		match input.trim() {
			"" => {
				user.click();
				println!("{}", user.score());
			},	
			_ => process::exit(0),
		}
	}
}
