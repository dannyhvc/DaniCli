use clap::Parser;
use dani_cli_metadata::DaniCliArgs;

fn main() {
    let args = DaniCliArgs::parse();

    match args {
        // example: cargo run --bin dani_cli pathy unix ".\dani_cli\"
        DaniCliArgs::Pathy(p) => pathy::handle(p).unwrap(),
        DaniCliArgs::Zipper(z) => zipper::handle(z).unwrap(),
    }
}
