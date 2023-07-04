use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub enum ZipperArgs {
    Seperate {
        /// zipping the cwd all files seperately
        #[clap(index = 1)]
        cwd: String,
    },
    Join {
        /// zipping the cwd
        #[clap(index = 1)]
        cwd: String,
    },
    Select {
        /// current working directroy
        cwd: String,
        /// files to add
        items: Vec<String>,
    },
}

