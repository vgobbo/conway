use std::time::Duration;

use args::{Args, Commands, PresetArgs, Presets, RandomArgs, RenderMode};
use automata::AutomataCell;
use board_renderer::BoardRenderer;
use clap::Parser;
use finite_board::FiniteBoard;
use generators::{DiehardPatternGenerator, GosperGliderGunGenerator, SeededRandomGenerator};
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
		Presets::Diehard => Box::new(DiehardPatternGenerator::default()),
		Presets::Gosper => Box::new(GosperGliderGunGenerator::new(args.width, args.height)),
		Presets::StillBlock => Box::new(StillBlockGenerator::default()),
	};

	let object = generator.generate();
	let mut grid = generators::grids::create(
		args.width.unwrap_or(generator.width()),
		args.height.unwrap_or(generator.height()),
	);
	generators::grids::translate_into(0, 0, object, &mut grid);

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
