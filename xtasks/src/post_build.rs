pub fn main() {
    let staging_dir: std::path::PathBuf = std::env::var("TRUNK_STAGING_DIR")
        .expect("TRUNK_STAGING_DIR should be set").into();
    let data_dir = staging_dir.join("data");

    // copy "data" into "dist/" only if the feature pulled "data" first
    if fs::metadata("data").is_ok() {
        if let Err(e) = recursive_copy("data", &data_dir) {
            eprintln!("error: {e:?}");
        }
    }
}

use std::path::Path;
use std::{io, fs};

fn recursive_copy(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            recursive_copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
