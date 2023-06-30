use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub enum ZipperArgs {
    Seperate,
    Join,
    Select { items: Vec<String> },
}
