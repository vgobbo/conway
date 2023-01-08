use board_renderer::BoardRenderer;
use clap::{Parser, Subcommand};
use finite_board::FiniteBoard;
use generators::SeededRandomGenerator;
use simple_solver::SimpleSolver;
use solver::{SimpleCellProcessor, Thresholds};

use crate::{
	board::Render,
	generators::{Generator, RandomGenerator, StillBlockGenerator},
};

mod automata;
mod board;
mod board_renderer;
mod finite_board;
mod generators;
mod simple_solver;
mod solver;

#[derive(Subcommand, Clone)]
enum Commands {
	Random(RandomArgs),
	Preset(PresetArgs),
}

#[derive(Parser, Clone)]
struct RandomArgs {
	#[arg(long)]
	pub width: usize,

	#[arg(long)]
	pub height: usize,

	#[arg(long)]
	pub probability: f32,

	#[arg(long)]
	pub seed: Option<u64>,
}

#[derive(Parser, Clone)]
struct PresetArgs {
	#[arg(long)]
	pub name: String,
}

#[derive(Parser)]
struct Args {
	#[command(subcommand)]
	pub command: Commands,
}

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
	let renderer = BoardRenderer::new("1", "0", &board);
	let processor = SimpleCellProcessor::new(Thresholds::default());
	let solver = SimpleSolver::new(processor, &board);

	renderer.render();
	println!();
	loop {
		if !solver.next() {
			break;
		}
		board.swap();
		renderer.render();
		println!();
	}
}