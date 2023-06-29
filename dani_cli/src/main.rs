extern crate dani_cli_metadata;
extern crate pathy;
extern crate to_seperate_zips;

use clap::Parser;
use dani_cli_metadata::DaniCliArgs;



// math.rs
#[derive(Parser)]
pub struct MathCommand {
    #[clap(subcommand)]
    pub subcommand: MathSubcommand,
}

#[derive(Parser)]
pub enum MathSubcommand {
    Add(AddCommand),
}

pub fn handle_math(args: MathCommand) {
    match args.subcommand {
        MathSubcommand::Add(add_args) => handle_add(add_args),
    }
}

// add.rs
#[derive(Parser)]
pub struct AddCommand {
    #[clap(index = 1)]
    x: i32,
    #[clap(index=2)]
    y: i32,
}

pub fn handle_add(args: AddCommand) {
    let result = args.x + args.y;
    println!("{} + {} = {}", args.x, args.y, result);
}

// main.rs
#[derive(Parser)]
enum Cli {
    Math(MathCommand),
}
fn main() {
    let args = Cli::parse();
    match args {
        Cli::Math(math_args) => handle_math(math_args),
    }
}
