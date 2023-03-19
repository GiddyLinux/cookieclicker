#[derive(Default)]
pub struct User {
	score: u32,
}

impl User {
	/// Returns score of user.
	pub fn score(&mut self) -> u32 {
		self.score
	}
	
	/// Increments score by 1.
	pub fn click(&mut self) {
		self.score += 1;
	}
}

#[cfg(test)]
mod test {
	use super::*;

	// Check if click increments score by 1.
	#[test]
	fn click() {
		let mut user = User::default();
		let starting_score = user.score;
		user.click();
		let new_score = user.score;
		assert_eq!(new_score, starting_score + 1);
	}

	// Check if score returns score at 0 initially.
	#[test]
	fn score() {
		let user = User::default();
		assert_eq!(user.score, 0)	
	}
} 
