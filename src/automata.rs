use std::cell::Cell;

#[derive(Clone)]
pub struct AutomataCell {
	alive: Cell<bool>,
}

impl AutomataCell {
	pub fn new(alive: bool) -> AutomataCell {
		AutomataCell {
			alive: Cell::new(alive),
		}
	}

	pub fn is_alive(&self) -> bool {
		self.alive.get()
	}

	pub fn die(&self) {
		self.alive.set(false);
	}

	pub fn live(&self) {
		self.alive.set(true);
	}
}

impl From<bool> for AutomataCell {
	fn from(value: bool) -> Self {
		AutomataCell::new(value)
	}
}

impl From<u8> for AutomataCell {
	fn from(value: u8) -> Self {
		AutomataCell::new(value != 0)
	}
}
