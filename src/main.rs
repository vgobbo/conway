use args::{Args, Commands, RandomArgs, RenderMode};
use board_renderer::BoardRenderer;
use clap::Parser;
use finite_board::FiniteBoard;
use generators::SeededRandomGenerator;
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

fn build_random_generator(args: RandomArgs) -> Box<dyn Generator> {
	if let Some(seed) = args.seed {
		Box::new(SeededRandomGenerator::new(
			args.width,
			args.height,
			seed,
			args.probability,
		))
	} else {
		Box::new(RandomGenerator::new(args.width, args.height, args.probability))
	}
}

fn main() {
	let args = Args::parse();

	let generator: Box<dyn Generator> = match args.command {
		Commands::Random(args) => build_random_generator(args),
		Commands::Preset(_) => Box::new(StillBlockGenerator::default()),
	};

	let board = FiniteBoard::new(generator);
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
			renderer.render();
			println!();
		}
	}

	if args.render_mode == RenderMode::Final {
		renderer.render();
		println!();
	}
}
