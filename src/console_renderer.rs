use crate::board::{IndexedRow, Render};

pub struct ConsoleRenderer<'a, T>
where
	T: IndexedRow,
{
	board: &'a T,
	alive_glyph: &'a str,
	dead_glyph: &'a str,
}

impl<'a, T> ConsoleRenderer<'a, T>
where
	T: IndexedRow,
{
	pub fn new(alive_glyph: &'a str, dead_glyph: &'a str, board: &'a T) -> ConsoleRenderer<'a, T> {
		ConsoleRenderer {
			board: board,
			alive_glyph,
			dead_glyph,
		}
	}
}

impl<'a, T> Render for ConsoleRenderer<'a, T>
where
	T: IndexedRow,
{
	fn render(&self) {
		let mut lines: Vec<String> = Vec::with_capacity(self.board.row_count());

		for i in 0..self.board.row_count() {
			let line = self
				.board
				.main_row(i)
				.iter()
				.map(|cell| {
					if cell.is_alive() {
						self.alive_glyph
					} else {
						self.dead_glyph
					}
				})
				.collect::<Vec<&str>>()
				.join("");
			lines.push(line);
		}

		println!("{}", termion::cursor::Goto(1, 1));
		println!("{}", lines.join("\n"));
	}
}
