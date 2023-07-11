use std::path::Path;
use std::path::PathBuf;
use std::{
    fs::{self, File},
    io,
};
use zip::{write::FileOptions, ZipWriter};

/// Compresses each file and directory within the specified directory into separate ZIP archives.
///
/// # Arguments
/// * `wd` - The path to the directory to be zipped.
///
/// # Errors
/// This function may return an `io::Error` if any I/O operations fail.
///
/// # Brief
/// The function takes a directory path (`wd`) and creates separate ZIP archives for each file and directory within the specified directory. Each ZIP archive is named after its corresponding file or directory. The resulting ZIP archives are saved to disk.
/// The function returns `io::Result<()>` indicating success or an `io::Error` if any I/O operations fail.
///
/// The example demonstrates how to use the `_zip_all_separately` function. It creates a temporary directory, writes a file to it, and then calls `_zip_all_separately` with the temporary directory path. After the function call, it asserts that the ZIP archive for the file exists and verifies the contents of the file within the archive.
/// The assertions help ensure that the function behaves as expected.
pub(crate) fn _zip_all_serperately(wd: String) -> io::Result<()> {
    let path = Path::new(wd.as_str());
    let entries: Vec<PathBuf> = fs::read_dir(path)?
        .map(|entry| entry.unwrap().path())
        .collect::<Vec<_>>();

    for entry in entries {
        let name: &str = entry.file_name().unwrap().to_str().unwrap();
        let zip_name: String = format!("{}.zip", name);
        let file: File = fs::File::create(zip_name)?;
        let mut zip: ZipWriter<File> = ZipWriter::new(file);
        let options = FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .unix_permissions(0o755);

        let mut files = Vec::new();
        __get_files(&entry, &mut files);

        for file in files {
            let name = file.strip_prefix(path).unwrap().to_str().unwrap();
            if file.is_dir() {
                zip.add_directory(name, options)?;
            } else {
                zip.start_file(name, options)?;
                let mut f = fs::File::open(file)?;
                io::copy(&mut f, &mut zip)?;
            }
        }

        zip.finish()?;
    }

    Ok(())
}

/// Compresses all files and directories within the specified directory into a ZIP archive.
///
/// # Arguments
/// * `wd` - The path to the directory to be zipped.
/// * `name` - An optional name for the resulting ZIP archive. If `Some`, the archive will be named as `name.zip`. If `None`, the default name "ARCHIVE.zip" will be used.
///
/// # Errors
/// This function may return an `io::Error` if any I/O operations fail.
///
/// # Brief
/// The function takes a directory path (`wd`) and an optional archive name (`name`), and creates a ZIP archive containing all files and directories within the specified directory. The resulting ZIP archive is saved to disk. If an archive name is provided, it will be used; otherwise, a default name ("ARCHIVE.zip") will be used.
/// The function returns `io::Result<()>` indicating success or an `io::Error` if any I/O operations fail.
///
/// The example demonstrates how to use the `_zip_all_joined` function. It creates a temporary directory, writes a file to it, and then calls `_zip_all_joined` with the temporary directory path and a custom archive name. After the function call, it asserts that the archive file exists and verifies the contents of a specific file within the archive.
/// The assertions help ensure that the function behaves as expected.
pub(crate) fn _zip_all_joined(wd: String, name: Option<String>) -> io::Result<()> {
    let path = Path::new(wd.as_str());
    let mut files = Vec::new();
    __get_files(&path, &mut files);

    let file: File = match name {
        Some(x) => fs::File::create(x + ".zip")?,
        None => fs::File::create("ARCHIVE.zip")?,
    };
    let mut zip: ZipWriter<fs::File> = ZipWriter::new(file);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    for file in files {
        let name: &str = file.strip_prefix(path).unwrap().to_str().unwrap();
        if file.is_dir() {
            zip.add_directory(name, options)?;
        } else {
            zip.start_file(name, options)?;
            let mut f = fs::File::open(file)?;
            io::copy(&mut f, &mut zip)?;
        }
    }

    zip.finish()?;
    Ok(())
}

pub(crate) fn _zip_selected_files(wd: String, selection: Option<Vec<String>>) -> io::Result<()> {
    // TODO: zip each selected file seperately but don't touch any other file in the directory that
    // is not listed.
    todo!();
}

fn __get_files(path: &Path, files: &mut Vec<PathBuf>) {
    if path.is_dir() {
        files.push(path.to_path_buf());
        fs::read_dir(path).unwrap().for_each(|entry| {
            let entry: fs::DirEntry = entry.unwrap();
            let path: PathBuf = entry.path();
            __get_files(&path, files);
        });
    } else {
        files.push(path.to_path_buf());
    }
}
