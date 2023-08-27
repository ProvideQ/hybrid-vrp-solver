use clap::{Args, Parser, Subcommand, ValueEnum};

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
    /// clustering config
    #[arg(value_enum)]
    pub cluster: ClusterOption,
    #[arg(short = 'n', long, default_value_t = 3)]
    pub cluster_number: usize,
    #[arg(value_enum)]
    pub solver: SolverOption,
    #[arg(short = 'd', long, default_value_t = String::from("./.vrp"))]
    pub build_dir: String,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ClusterOption {
    Knn,
    Tsp,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum SolverOption {
    Lkh,
    Simulated,
    LeapHybrid,
    QbSolv,
    Direct,
}
