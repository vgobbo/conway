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
		grids::to_vec(&grids::STILL_BLOCK_GRID)
	}

	fn height(&self) -> usize {
		grids::STILL_BLOCK_MIN_HEIGHT
	}

	fn width(&self) -> usize {
		grids::STILL_BLOCK_MIN_WIDTH
	}
}

pub struct GosperGliderGunGenerator {
	width: usize,
	height: usize,
}

impl GosperGliderGunGenerator {
	pub fn new(width: Option<usize>, height: Option<usize>) -> GosperGliderGunGenerator {
		const MIN_WIDTH: usize = 38;
		const MIN_HEIGHT: usize = 11;

		let w = width.unwrap_or(MIN_WIDTH);
		let h = height.unwrap_or(MIN_HEIGHT);
		if w < MIN_WIDTH || h < MIN_HEIGHT {
			panic!("Grid {w}x{h} is too small (min. {MIN_WIDTH}x{MIN_HEIGHT}).");
		}

		GosperGliderGunGenerator { width: w, height: h }
	}
}

impl Default for GosperGliderGunGenerator {
	fn default() -> Self {
		GosperGliderGunGenerator::new(None, None)
	}
}

impl Generator for GosperGliderGunGenerator {
	fn generate(&self) -> Vec<Vec<AutomataCell>> {
		grids::to_vec(grids::GOSPER_GLIDER_GUN_GRID)
	}

	fn height(&self) -> usize {
		self.height
	}

	fn width(&self) -> usize {
		self.width
	}
}

#[rustfmt::skip]
pub mod grids {
    use crate::automata::AutomataCell;

	pub const GOSPER_GLIDER_GUN_MIN_WIDTH: usize = 38;
	pub const GOSPER_GLIDER_GUN_MIN_HEIGHT: usize = 11;
	pub const GOSPER_GLIDER_GUN_GRID: [[u8;GOSPER_GLIDER_GUN_MIN_WIDTH]; GOSPER_GLIDER_GUN_MIN_HEIGHT] = [
		[ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
		[ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
		[ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
		[ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, ],
		[ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, ],
		[ 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
		[ 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
		[ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
		[ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
		[ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
		[ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
	];

	pub const STILL_BLOCK_MIN_WIDTH: usize = 4;
	pub const STILL_BLOCK_MIN_HEIGHT: usize = 4;
	pub const STILL_BLOCK_GRID: [[u8;STILL_BLOCK_MIN_WIDTH]; STILL_BLOCK_MIN_HEIGHT] = [
		[ 0, 0, 0, 0, ],
		[ 0, 1, 1, 0, ],
		[ 0, 1, 1, 0, ],
		[ 0, 0, 0, 0, ],
	];

	pub fn to_vec<G: AsRef<[R]>, R: AsRef<[u8]>>(grid: G) -> Vec<Vec<AutomataCell>> {
		let grid = grid.as_ref();
		grid
			.iter()
			.map(|row| row.as_ref().iter().map(|cell| cell.clone().into()).collect())
			.collect()
	}

	pub fn create(width: usize, height: usize) -> Vec<Vec<AutomataCell>> {
		(0..height)
			.map(|_| {
				let mut row = Vec::with_capacity(0);
				Vec::resize_with(&mut row, width, || AutomataCell::new(false));
				row
			})
			.collect()
	}

	// There is a compiler bug that warns about unused_mut even though it is required.
	#[allow(unused_mut)]
	pub fn translate_into(di: i32, dj: i32, object: Vec<Vec<AutomataCell>>, grid: &mut Vec<Vec<AutomataCell>>) {
		let obj_w = object.get(0).unwrap().len();
		let obj_h = object.len();

		for i in 0..obj_h {
			let obj_row = object.get(i).unwrap();
			let mut grid_row = grid.get_mut((i as i32 + di) as usize).unwrap();
			for j in 0..obj_w {
				let cell = grid_row.get_mut((j as i32 + dj) as usize).unwrap();
				*cell = obj_row.get(j).unwrap().clone();
			}
		}
	}
}
