use std::path::PathBuf;
use ui_common::IconFiles;

fn main() -> Result<(), std::io::Error> {
    let path = PathBuf::from(
        std::env::var("TRUNK_STAGING_DIR").expect("Environment variable TRUNK_STAGING_DIR"),
    );
    if !path.is_dir() {
        std::fs::create_dir(&path)?;
    }
    IconFiles::FILES.copy(&path)
}
