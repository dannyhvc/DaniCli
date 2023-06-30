use blueprint::ZipperArgs;

pub mod blueprint;

pub fn handle(args: ZipperArgs) -> Result<(), String> {
    match args {
        ZipperArgs::Seperate => todo!(),
        ZipperArgs::Join => todo!(),
        ZipperArgs::Select { items } => todo!(),
    }
    Ok(())
}
