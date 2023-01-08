use rand::{Rng, SeedableRng};

use crate::automata::AutomataCell;

pub trait Generator {
	fn generate(&self) -> Vec<Vec<AutomataCell>>;
	fn width(&self) -> usize;
	fn height(&self) -> usize;
}

pub struct RandomGenerator {
	width: usize,
	height: usize,
	probability: f32,
}

impl RandomGenerator {
	pub fn new(width: usize, height: usize, probability: f32) -> RandomGenerator {
		RandomGenerator {
			width,
			height,
			probability,
		}
	}
}

impl Generator for RandomGenerator {
	fn generate(&self) -> Vec<Vec<AutomataCell>> {
		let mut rng = rand::thread_rng();

		let buffer: Vec<Vec<AutomataCell>> = (0..self.height)
			.map(|_| {
				let mut row = Vec::with_capacity(0);
				Vec::resize_with(&mut row, self.width, || {
					AutomataCell::new(rng.gen::<f32>() < self.probability)
				});
				row
			})
			.collect();

		buffer
	}

	fn height(&self) -> usize {
		self.height
	}

	fn width(&self) -> usize {
		self.width
	}
}

pub struct SeededRandomGenerator {
	width: usize,
	height: usize,
	seed: u64,
	probability: f32,
}

impl SeededRandomGenerator {
	pub fn new(width: usize, height: usize, seed: u64, probability: f32) -> SeededRandomGenerator {
		SeededRandomGenerator {
			width,
			height,
			seed,
			probability,
		}
	}
}

impl Generator for SeededRandomGenerator {
	fn generate(&self) -> Vec<Vec<AutomataCell>> {
		let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(self.seed);

		let buffer: Vec<Vec<AutomataCell>> = (0..self.height)
			.map(|_| {
				let mut row = Vec::with_capacity(0);
				Vec::resize_with(&mut row, self.width, || {
					AutomataCell::new(rng.gen::<f32>() < self.probability)
				});
				row
			})
			.collect();

		buffer
	}

	fn height(&self) -> usize {
		self.height
	}

	fn width(&self) -> usize {
		self.width
	}
}

pub struct StillBlockGenerator {}

impl Default for StillBlockGenerator {
	fn default() -> Self {
		StillBlockGenerator {}
	}
}

impl Generator for StillBlockGenerator {
	fn generate(&self) -> Vec<Vec<AutomataCell>> {
		vec![
			vec![false.into(), false.into(), false.into(), false.into()],
			vec![false.into(), true.into(), true.into(), false.into()],
			vec![false.into(), true.into(), true.into(), false.into()],
			vec![false.into(), false.into(), false.into(), false.into()],
		]
	}

	fn height(&self) -> usize {
		4
	}

	fn width(&self) -> usize {
		4
	}
}
