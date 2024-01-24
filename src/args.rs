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
    /// conduct part of the solving step
    Partial(PartialSolveCommand),
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

#[derive(Debug, Args)]
pub struct PartialSolveCommand {
    #[clap(subcommand)]
    pub subcommand: PartialSolveSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum PartialSolveSubCommand {
    Cluster(OnlyClusterCommand),
    Solve(OnlySolveCommand),
}

#[derive(Debug, Args)]
pub struct OnlyClusterCommand {
    /// CVRP-tsplib file path or a folder containing CVRP-tsplib instances
    pub path: String,
    /// clustering config
    #[arg(value_enum)]
    pub cluster: ClusterOption,
    #[arg(short = 'n', long, default_value_t = 3)]
    pub cluster_number: usize,
    #[arg(short = 'd', long, default_value_t = String::from("./.vrp"))]
    pub build_dir: String,
}

#[derive(Debug, Args)]
pub struct OnlySolveCommand {
    /// CVRP-tsplib file path or a folder containing CVRP-tsplib instances
    pub path: String,
    #[arg(value_enum)]
    pub solver: SolverOption,
    #[arg(long, default_value_t = false)]
    pub transform_only: bool,
    #[arg(short = 'd', long, default_value_t = String::from("./.vrp"))]
    pub build_dir: String,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ClusterOption {
    Kmeans,
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
