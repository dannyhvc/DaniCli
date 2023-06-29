use clap::Parser;
use pathy::pathy_start::Pathy;

/// Simple program to greet a person
#[derive(Parser, Debug, Clone)]
#[clap(
    name = "DaniCliArgs",
    about = "A bunch of simplified blazing fast cli tools"
)]
pub enum DaniCliArgs {
    Path(Pathy)
}