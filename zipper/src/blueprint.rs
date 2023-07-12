use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub enum ZipperArgs {
    Seperate {
        /// zipping the working directory all files seperately
        #[clap(default_value = ".")]
        wd: String,
    },
    Join {
        /// zipping the working directory
        #[clap(default_value = ".")]
        wd: String,
        #[arg(short='n', default_value=None)]
        name: Option<String>,
    },
    Select {
        /// current working directroy
        #[clap(default_value = ".")]
        wd: String,
        /// files to add
        #[arg(short = 'l', default_value=None)]
        list: Option<Vec<String>>,
    },
}
