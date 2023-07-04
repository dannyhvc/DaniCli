use std::io::{prelude::*, self};
use zip::ZipWriter;

use std::iter::Iterator;
use zip::write::FileOptions;

use std::fs::{DirEntry, File, self};
use std::path::Path;

pub(crate) fn _zip_all_serperately(cwd: String) {}

pub(crate) fn _zip_all_joined(cwd: String) -> io::Result<()> {
    let path = Path::new(cwd.as_str());
    let files = fs::read_dir(path)?
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_file())
        .collect::<Vec<_>>();

    let file = fs::File::create("archive.zip")?;
    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    for file in files {
        let name = file.file_name().unwrap().to_str().unwrap();
        zip.start_file(name, options)?;
        let mut f = fs::File::open(file)?;
        io::copy(&mut f, &mut zip)?;
    }

    zip.finish()?;
    Ok(())
}

pub(crate) fn _zip_selected_files(cwd: String, selection: Vec<String>) {}
