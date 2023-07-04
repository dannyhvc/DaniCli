use blueprint::ZipperArgs;
pub mod blueprint;
pub(crate) mod helpers;

pub fn handle(args: ZipperArgs) -> Result<(), String> {
    match args {
        ZipperArgs::Seperate { cwd } => {
            helpers::_zip_all_serperately(cwd);
        }
        ZipperArgs::Join { cwd } => {
            helpers::_zip_all_joined(cwd);
        }
        ZipperArgs::Select { cwd, items } => {
            helpers::_zip_selected_files(cwd, items);
        }
    }
    Ok(())
}
