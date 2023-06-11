mod args;

use args::{VRPCommand, VRPSolverArgs};
use clap::Parser;

fn main() {
    let args = VRPSolverArgs::parse();

    match args.command {
        VRPCommand::Solve(subcommandargs) => println!("path: {}", subcommandargs.path),
    }
}
