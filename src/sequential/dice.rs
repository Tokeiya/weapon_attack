use std::vec::Vec;

pub struct Dice {
	values: Vec<u32>,
	current: usize,
}

impl Dice {
	pub fn new(values: Vec<u32>) -> Dice {
		Dice { values, current: 0 }
	}

	pub fn current(&self) -> u32 {
		self.values[self.current]
	}

	pub fn next(&mut self) -> (bool, u32) {
		self.current += 1;
		if self.current >= self.values.len() {
			self.current = 0;
			(true, self.values[self.current])
		} else {
			(false, self.values[self.current])
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_dice() {
		let mut dice = Dice::new(vec![1, 2, 3, 4, 5]);
		assert_eq!(dice.next(), (false, 2));
		assert_eq!(dice.next(), (false, 3));
		assert_eq!(dice.next(), (false, 4));
		assert_eq!(dice.next(), (false, 5));
		assert_eq!(dice.next(), (true, 1));
	}

	#[test]
	fn test_dice_initial_value() {
		let dice = Dice::new(vec![1, 2, 3, 4, 5]);
		assert_eq!(dice.current(), 1);
	}
}
