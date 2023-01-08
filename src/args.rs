use std::{fmt::Display, str::FromStr};

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Subcommand, Clone)]
pub enum Commands {
	Random(RandomArgs),
	Preset(PresetArgs),
}

#[derive(Parser, Clone)]
pub struct RandomArgs {
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
pub struct PresetArgs {
	#[arg(long)]
	pub width: Option<usize>,

	#[arg(long)]
	pub height: Option<usize>,

	#[arg(long)]
	pub name: Presets,
}

#[derive(Clone, Debug, ValueEnum, PartialEq)]
pub enum Presets {
	Gosper,
	StillBlock,
}

impl Display for Presets {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Presets::Gosper => write!(f, "gosper"),
			Presets::StillBlock => write!(f, "still-block"),
		}
	}
}

impl FromStr for Presets {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"gosper" => Ok(Presets::Gosper),
			"still-block" => Ok(Presets::StillBlock),
			_ => Err(format!("Unknown preset: {s}.")),
		}
	}
}

#[derive(Clone, Debug, ValueEnum, PartialEq)]
pub enum RenderMode {
	All,
	Final,
	None,
}

impl Display for RenderMode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			RenderMode::All => write!(f, "all"),
			RenderMode::Final => write!(f, "final"),
			RenderMode::None => write!(f, "none"),
		}
	}
}

impl FromStr for RenderMode {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"all" => Ok(RenderMode::All),
			"final" => Ok(RenderMode::Final),
			"none" => Ok(RenderMode::None),
			_ => Err(format!("Unknown render mode: {s}.")),
		}
	}
}

#[derive(Parser)]
pub struct Args {
	#[arg(long, default_value_t = std::u64::MAX)]
	pub max_iterations: u64,

	#[arg(long, default_value = "O")]
	pub alive_glyph: String,

	#[arg(long, default_value = " ")]
	pub dead_glyph: String,

	#[arg(long, default_value_t = RenderMode::Final, value_parser = clap::builder::EnumValueParser::<RenderMode>::new())]
	pub render_mode: RenderMode,

	#[command(subcommand)]
	pub command: Commands,
}