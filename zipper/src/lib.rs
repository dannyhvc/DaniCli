use std::io;

use blueprint::ZipperArgs;
pub mod blueprint;
pub(crate) mod helpers;

pub fn handle(args: ZipperArgs) -> io::Result<()> {
    match args {
        ZipperArgs::Seperate { wd } => helpers::_zip_all_serperately(wd),
        ZipperArgs::Join { wd, name } => helpers::_zip_all_joined(wd, name),
        ZipperArgs::Select { wd, items } => helpers::_zip_selected_files(wd, items),
    }
}
