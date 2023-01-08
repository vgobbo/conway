use crate::{
	board::{Board, IndexedRow},
	solver::CellProcessor,
};

pub struct SimpleSolver<'a, T, P>
where
	T: Board + IndexedRow,
	P: CellProcessor,
{
	board: &'a T,
	processor: P,
}

impl<'a, T, P> SimpleSolver<'a, T, P>
where
	T: Board + IndexedRow,
	P: CellProcessor,
{
	pub fn new(processor: P, board: &'a T) -> SimpleSolver<'a, T, P> {
		SimpleSolver { board, processor }
	}

	pub fn next(&self) -> bool {
		let mut modified = false;

		for i in 0..self.board.row_count() {
			let main_row = self.board.main_row(i);
			let back_row = self.board.back_row(i);

			for j in 0..main_row.len() {
				let alive_neighbors = self.board.count_alive_neighbors(i, j);

				let main_cell = main_row.get(j).unwrap();
				let back_cell = back_row.get(j).unwrap();

				if self.processor.process(main_cell, back_cell, alive_neighbors, 0) {
					modified = true;
				}
			}
		}

		modified
	}
}
