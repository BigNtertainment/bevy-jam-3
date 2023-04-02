use bevy::prelude::*;

use crate::pill::Pill;

#[derive(Component, Clone, Debug, PartialEq)]
pub struct Inventory {
	pills: Vec<Pill>,
	capacity: usize,
}

#[allow(unused)]
impl Inventory {
	pub fn new(capacity: usize) -> Self {
		Self {
			pills: Vec::with_capacity(capacity),
			capacity,
		}
	}

	#[must_use]
	pub fn add_pill(&mut self, pill: Pill) -> bool {
		if self.pills.len() < self.capacity {
			self.pills.push(pill);
			true
		} else {
			false
		}
	}

	pub fn remove_pill(&mut self, index: usize) -> Pill {
		self.pills.remove(index)
	}

	pub fn get_pill(&self, index: usize) -> Option<&Pill> {
		self.pills.get(index)
	}

	pub fn get_pills(&self) -> &[Pill] {
		&self.pills
	}

	pub fn get_capacity(&self) -> usize {
		self.capacity
	}

	pub fn get_pill_count(&self) -> usize {
		self.pills.len()
	}
}