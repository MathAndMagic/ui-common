//! # Stylesheets
//!
//! To use stylesheets, you should deal with its file. This file can be added by:
//!
//! 1. Using [`Stylesheet::copy()`](crate::StylesheetFiles::copy) - in build-executed code (recommended, see below)
//! 2. Accessing the data via [`StylesheetFiles::FILES`] and delivering it yourself
//!
//! # 1. Recommended - Automatically Copying Files
//!
//! This is copies the file to the `dist` directory, which is recommended.
//!
//! 1. `Cargo.toml`
//!
//!    Add the build-executed binary.
//!
//!    ```toml
//!    [[bin]]
//!    name = "copy-assets"
//!    ```
//!
//! 2. `src/bin/copy-assets.rs`
//!
//!    Create the program to copy the files.
//!
//!    ```no_run
//!    use std::path::PathBuf;
//!    use ui_common::StylesheetFiles;
//!
//!    fn main() -> Result<(), std::io::Error> {
//!        let path = PathBuf::from(
//!            std::env::var("TRUNK_STAGING_DIR").expect("Environment variable TRUNK_STAGING_DIR"),
//!        );
//!        if !path.is_dir() {
//!            std::fs::create_dir(&path)?;
//!        }
//!
//!        StylesheetFiles::copy(&path)
//!    }
//!    ```
//!
//! 3. `index.html`
//!
//!    Set base reference, link the required CSS and specify your WASM program[^1].
//!
//!    [^1]: Since we'll be writing a build-executed program, there are now two binaries and trunk needs to know which is your WASM binary.
//!
//!    ```html
//!    <base data-trunk-public-url />
//!    <link rel="stylesheet" href="common/tailwind.css" />
//!    <link data-trunk rel="rust" data-bin="name-of-app" />
//!    ```
//!
//! 4. `Trunk.toml`
//!
//!    Add a hook to run the build-executed program.
//!
//!    ```toml
//!    [[hooks]]
//!    stage = "build"
//!    command = "cargo"
//!    command_arguments = ["run", "--bin", "copy-assets"]
//!    ```
//!

use std::fs;
pub struct StylesheetFiles {
    pub tailwind: &'static str,
}

impl StylesheetFiles {
    pub const FILES: Self = Self {
        tailwind: include_str!(concat!(env!("OUT_DIR"), "/tailwind_generated.css")),
    };

    /// Copy stylesheets to the specified directory.
    ///
    /// # Errors
    ///
    /// Will return an error when there is a problem with writing the files or creating the directory.
    pub fn copy(to: &std::path::Path) -> Result<(), std::io::Error> {
        let StylesheetFiles { tailwind } = Self::FILES;

        let to = to.join("common");
        if !to.is_dir() {
            fs::create_dir(&to)?;
        }

        fs::write(to.join("tailwind.css"), tailwind)
    }
}
