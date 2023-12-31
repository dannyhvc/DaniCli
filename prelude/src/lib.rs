use clap::Parser;
use pathy::blueprint::PathyArgs;
use zipper::blueprint::ZipperArgs;

/// Simple program to greet a person
#[derive(Parser, Debug, Clone)]
#[clap(
    name = "DaniCliArgs",
    about = "A bunch of simplified blazing fast cli tools"
)]
pub enum DaniCliArgs {
    #[clap(subcommand)]
    Pathy(PathyArgs),

    #[clap(subcommand)]
    Zipper(ZipperArgs),
}
