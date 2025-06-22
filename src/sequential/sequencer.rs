use crate::sequential::dice::Dice;
use std::slice::Iter;
use std::vec::Vec;

pub struct Sequencer {
	dices: Vec<Dice>,
}

impl Sequencer {
	pub fn new(dices: Vec<Dice>) -> Sequencer {
		Sequencer { dices }
	}

	pub fn create(source: Iter<i32>,count:usize) -> Sequencer {
		let dice: Vec<i32> = source.cloned().collect();
		
		let mut vec=Vec::<Dice>::new();
		
		for _ in 0..count{
			vec.push(Dice::new(dice.clone()));
		}

		Sequencer::new(vec)
	}

	pub fn get_current(&self, buff: &mut [i32]) {
		let iter = self.dices.iter().zip(buff.iter_mut());
		for (d, b) in iter {
			*b = d.current()
		}
	}

	pub fn aggregate<T>(&self, f: impl Fn(&[i32]) -> T) -> T {
		let mut arr = Vec::<i32>::new();
		arr.resize(self.dices.len(), 0);
		self.get_current(&mut arr);
		f(&arr)
	}

	pub fn move_next(&mut self) -> bool {
		for d in self.dices.iter_mut() {
			let (carry, _) = d.next();
			if !carry {
				return false;
			}
		}

		true
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_create() {
		let source = vec![0, 1, 2, 3, 4, 5];
		let seq = Sequencer::create(source.iter(),2);
		let mut buff = [0i32; 2];

		seq.get_current(&mut buff);
		assert_eq!(buff, [0, 0]);
	}

	#[test]
	fn new_test() {
		let a = Dice::new(vec![1, 2, 3, 4, 5, 6]);
		let b = Dice::new(vec![0, 1, 2, 3, 4, 5]);
		let fixture = Sequencer::new(vec![a, b]);

		let mut buff = [0i32; 2];
		fixture.get_current(&mut buff);
		assert_eq!(buff, [1, 0]);
	}

	#[test]
	fn move_next_test() {
		let a = Dice::new(vec![1, 2]);
		let b = Dice::new(vec![0, 1]);
		let mut fixture = Sequencer::new(vec![a, b]);

		let mut buff = [0i32; 2];

		fixture.get_current(&mut buff);
		assert_eq!(buff, [1, 0]);

		assert!(!fixture.move_next());
		fixture.get_current(&mut buff);
		assert_eq!(buff, [2, 0]);

		assert!(!fixture.move_next());
		fixture.get_current(&mut buff);
		assert_eq!(buff, [1, 1]);

		assert!(!fixture.move_next());
		fixture.get_current(&mut buff);
		assert_eq!(buff, [2, 1]);

		assert!(fixture.move_next());
		fixture.get_current(&mut buff);
		assert_eq!(buff, [1, 0]);
	}

	#[test]
	fn aggregate_test() {
		let a = Dice::new(vec![1, 2]);
		let b = Dice::new(vec![0, 1]);
		let mut fixture = Sequencer::new(vec![a, b]);

		let result = fixture.aggregate(|arr| arr.iter().sum::<i32>());
		assert_eq!(result, 1);

		fixture.move_next();
		assert_eq!(fixture.aggregate(|arr| arr.iter().sum::<i32>()), 2);
	}
}
