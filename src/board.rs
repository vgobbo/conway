use crate::automata::AutomataCell;

pub trait Board {
	fn count_alive_neighbors(&self, i: usize, j: usize) -> usize;
	fn is_alive(&self, i: usize, j: usize) -> bool;
}

pub trait IndexedRow {
	fn row_count(&self) -> usize;
	fn main_row(&self, i: usize) -> &Vec<AutomataCell>;
	fn back_row(&self, i: usize) -> &Vec<AutomataCell>;
}

pub trait Render {
	fn render(&self);
}
