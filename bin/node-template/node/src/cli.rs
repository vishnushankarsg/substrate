use clap::ArgEnum;

use sc_cli::CliConfiguration;

#[derive(ArgEnum, Copy, Debug, Clone, PartialEq)]
pub enum NodeProcessingRole {
    Aggregator,
	LogicProvider
}

impl std::str::FromStr for NodeProcessingRole {
	type Err = String;
	fn from_str(s: &str) -> Result<Self, String> {
		if s.eq_ignore_ascii_case("aggregator") {
			Ok(Self::Aggregator)
		} else if s.eq_ignore_ascii_case("logicprovider") {
			Ok(Self::LogicProvider)
		} else {
			Err(format!("Unknown string variant given for node-processing-role cli flag"))
		}
	}
}


#[derive(Debug, clap::Parser)]
pub struct Cli {
	#[clap(subcommand)]
	pub subcommand: Option<Subcommand>,

	#[clap(flatten)]
	pub run: RunCmd,
}

#[derive(Debug, clap::Parser)]
pub struct RunCmd {
	#[clap(flatten)]
	pub base: sc_cli::RunCmd,

	/// Run node as aggregator or logic provider.
	#[clap(long)]
	pub processing_role: NodeProcessingRole,
}

impl CliConfiguration for RunCmd {
	fn shared_params(&self) -> &sc_cli::SharedParams {
		&self.base.shared_params
	}

	fn database_params(&self) -> Option<&sc_cli::DatabaseParams> {
		Some(&self.base.database_params().unwrap())
	}
}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
	/// Key management cli utilities
	#[clap(subcommand)]
	Key(sc_cli::KeySubcommand),

	/// Build a chain specification.
	BuildSpec(sc_cli::BuildSpecCmd),

	/// Validate blocks.
	CheckBlock(sc_cli::CheckBlockCmd),

	/// Export blocks.
	ExportBlocks(sc_cli::ExportBlocksCmd),

	/// Export the state of a given block into a chain spec.
	ExportState(sc_cli::ExportStateCmd),

	/// Import blocks.
	ImportBlocks(sc_cli::ImportBlocksCmd),

	/// Remove the whole chain.
	PurgeChain(sc_cli::PurgeChainCmd),

	/// Revert the chain to a previous state.
	Revert(sc_cli::RevertCmd),

	/// The custom benchmark subcommand benchmarking runtime pallets.
	#[clap(name = "benchmark", about = "Benchmark runtime pallets.")]
	Benchmark(frame_benchmarking_cli::BenchmarkCmd),
}
