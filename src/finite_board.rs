use std::cell::Cell;

use crate::{
	automata::AutomataCell,
	board::{Board, IndexedRow},
};

pub struct FiniteBoard {
	width: usize,
	height: usize,
	buffers: [Vec<Vec<AutomataCell>>; 2],
	main_buffer: Cell<usize>,
	back_buffer: Cell<usize>,
}

impl FiniteBoard {
	pub fn new(grid: Vec<Vec<AutomataCell>>) -> FiniteBoard {
		let width = grid.get(0).unwrap().len();
		let height = grid.len();
		let grids = [grid.clone(), grid];

		FiniteBoard {
			width,
			height,
			buffers: grids,
			main_buffer: Cell::new(0),
			back_buffer: Cell::new(1),
		}
	}

	fn neighbors_coords(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
		let ii = i as i32;
		let jj = j as i32;
		[
			(ii - 1, jj - 1),
			(ii, jj - 1),
			(ii + 1, jj - 1),
			(ii - 1, jj),
			(ii + 1, jj),
			(ii - 1, jj + 1),
			(ii, jj + 1),
			(ii + 1, jj + 1),
		]
		.into_iter()
		.filter(|(i, j)| i >= &0 && j >= &0)
		.filter(|(i, j)| i < &(self.height as i32) && j < &(self.width as i32))
		.map(|(i, j)| (i as usize, j as usize))
		.collect()
	}

	pub fn swap(&self) {
		self.main_buffer.set((self.main_buffer.get() + 1) % self.buffers.len());
		self.back_buffer.set((self.back_buffer.get() + 1) % self.buffers.len());
	}
}

impl IndexedRow for FiniteBoard {
	fn row_count(&self) -> usize {
		self.height
	}

	fn main_row(&self, i: usize) -> &Vec<AutomataCell> {
		self.buffers[self.main_buffer.get()]
			.get(i)
			.unwrap_or_else(|| panic!("({}) row not found.", i))
	}

	fn back_row(&self, i: usize) -> &Vec<AutomataCell> {
		self.buffers[self.back_buffer.get()]
			.get(i)
			.unwrap_or_else(|| panic!("({}) row not found.", i))
	}
}

impl Board for FiniteBoard {
	fn count_alive_neighbors(&self, i: usize, j: usize) -> usize {
		let coords = self.neighbors_coords(i, j);
		coords.into_iter().filter(|(i, j)| self.is_alive(*i, *j)).count()
	}

	fn is_alive(&self, i: usize, j: usize) -> bool {
		self.main_row(i)
			.get(j)
			.unwrap_or_else(|| panic!("({},{}) cell not found.", i, j))
			.clone()
			.is_alive()
	}
}
