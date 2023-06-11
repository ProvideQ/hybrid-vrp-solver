use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct VRPSolverArgs {
    #[clap(subcommand)]
    pub command: VRPCommand,
}

#[derive(Debug, Subcommand)]
pub enum VRPCommand {
    /// solve a vrp instance
    Solve(SolveCommand),
}

#[derive(Debug, Args)]
pub struct SolveCommand {
    /// CVRP-tsplib file path or a folder containing CVRP-tsplib instances
    pub path: String,
}
