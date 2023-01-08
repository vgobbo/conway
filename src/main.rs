use std::time::Duration;

use args::{Args, Commands, PresetArgs, Presets, RandomArgs, RenderMode};
use automata::AutomataCell;
use board_renderer::BoardRenderer;
use clap::Parser;
use finite_board::FiniteBoard;
use generators::{
	AcornPatternGenerator, DiehardPatternGenerator, GosperGliderGunGenerator, PentaDecathlonPatternGenerator,
	SeededRandomGenerator,
};
use simple_solver::SimpleSolver;
use solver::{SimpleCellProcessor, Thresholds};

use crate::{
	board::Render,
	generators::{Generator, RandomGenerator, StillBlockGenerator},
};

mod args;
mod automata;
mod board;
mod board_renderer;
mod finite_board;
mod generators;
mod simple_solver;
mod solver;

fn build_random_generator(args: RandomArgs) -> Vec<Vec<AutomataCell>> {
	let generator: Box<dyn Generator> = match args.seed {
		Some(seed) => {
			Box::new(SeededRandomGenerator::new(
				args.width,
				args.height,
				seed,
				args.probability,
			))
		},
		None => Box::new(RandomGenerator::new(args.width, args.height, args.probability)),
	};
	generator.generate()
}

fn build_preset_generator(args: PresetArgs) -> Vec<Vec<AutomataCell>> {
	let generator: Box<dyn Generator> = match args.name {
		Presets::Acorn => Box::new(AcornPatternGenerator::default()),
		Presets::Diehard => Box::new(DiehardPatternGenerator::default()),
		Presets::Gosper => Box::new(GosperGliderGunGenerator::new(args.width, args.height)),
		Presets::PentaDecathlon => Box::new(PentaDecathlonPatternGenerator::default()),
		Presets::StillBlock => Box::new(StillBlockGenerator::default()),
	};

	let object = generator.generate();

	let grid_width = args.width.unwrap_or(generator.width());
	let grid_height = args.height.unwrap_or(generator.height());
	let mut grid = generators::grids::create(grid_width, grid_height);

	if args.center {
		let di = (grid_height - generator.height()) / 2;
		let dj = (grid_width - generator.width()) / 2;
		generators::grids::translate_into(di as i32, dj as i32, object, &mut grid);
	} else {
		generators::grids::translate_into(0, 0, object, &mut grid);
	}

	grid
}

fn main() {
	let args = Args::parse();

	let grid = match args.command {
		Commands::Random(args) => build_random_generator(args),
		Commands::Preset(args) => build_preset_generator(args),
	};

	let board = FiniteBoard::new(grid);
	let renderer = BoardRenderer::new(args.alive_glyph.as_str(), args.dead_glyph.as_str(), &board);
	let processor = SimpleCellProcessor::new(Thresholds::default());
	let solver = SimpleSolver::new(processor, &board);

	if args.render_mode != RenderMode::None {
		renderer.render();
		println!();
	}

	for _ in 0..args.max_iterations {
		if !solver.next() {
			break;
		}
		board.swap();
		if args.render_mode == RenderMode::All {
			if let Some(frame_delay) = &args.frame_delay {
				std::thread::sleep(Duration::from_millis(frame_delay.clone()));
			}
			renderer.render();
			println!();
		}
	}

	if args.render_mode == RenderMode::Final {
		renderer.render();
		println!();
	}
}
