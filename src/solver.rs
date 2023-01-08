use crate::automata::AutomataCell;

pub struct Thresholds {
	pub underpopulation: usize,
	pub overpopulation: usize,
	pub reproduction: usize,
}

pub trait CellProcessor {
	fn process(
		&self,
		main_cell: &AutomataCell,
		back_cell: &AutomataCell,
		alive_neighbors: usize,
		dead_neighbors: usize,
	) -> bool;
}

pub struct SimpleCellProcessor {
	thresholds: Thresholds,
}

impl Default for Thresholds {
	fn default() -> Self {
		Thresholds {
			underpopulation: 2,
			overpopulation: 3,
			reproduction: 3,
		}
	}
}

impl SimpleCellProcessor {
	pub fn new(thresholds: Thresholds) -> Self {
		SimpleCellProcessor { thresholds }
	}
}

impl CellProcessor for SimpleCellProcessor {
	fn process(&self, main_cell: &AutomataCell, back_cell: &AutomataCell, alive_neighbors: usize, _: usize) -> bool {
		if main_cell.is_alive() {
			if alive_neighbors > self.thresholds.overpopulation {
				back_cell.die();
				true
			} else if alive_neighbors < self.thresholds.underpopulation {
				back_cell.die();
				true
			} else {
				back_cell.live();
				false
			}
		} else if alive_neighbors == self.thresholds.reproduction {
			back_cell.live();
			true
		} else {
			back_cell.die();
			false
		}
	}
}
