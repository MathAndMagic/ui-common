//! # Stylesheets
//!
//! To use stylesheets, you should deal with its file. This file can be added by:
//!
//! 1. Using [`Stylesheet::copy()`](crate::Stylesheet::copy) - in build-executed code (recommended, see below)
//! 2. Accessing the data via [`Stylesheet::*`] and delivering it yourself
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
//!    name = "copy-stylesheets"
//!    ```
//!
//! 2. `src/bin/copy-stylesheets.rs`
//!
//!    Create the program to copy the files.
//!
//!    ```no_run
//!    use std::path::PathBuf;
//!    use ui_common::Stylesheet;
//!
//!    fn main() -> Result<(), std::io::Error> {
//!        let path = PathBuf::from(
//!            std::env::var("TRUNK_STAGING_DIR").expect("Environment variable TRUNK_STAGING_DIR"),
//!        );
//!        if !path.is_dir() {
//!            std::fs::create_dir(&path)?;
//!        }
//!
//!        Stylesheet::copy_all(&path)
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
//!    command_arguments = ["run", "--bin", "copy-stylesheets"]
//!    ```
//!

use std::fs;
pub struct Stylesheet {
    pub file_name: &'static str,
    pub content: &'static str,
}

impl Stylesheet {
    pub const TAILWIND: Self = Self {
        file_name: "tailwind.css",
        content: include_str!(concat!(env!("OUT_DIR"), "/tailwind_generated.css")),
    };

    /// Copy stylesheet to the specified directory.
    ///
    /// # Errors
    ///
    /// Will return an error when there is a problem with writing the files or creating the directory.
    pub fn copy(&self, to: &std::path::Path) -> Result<(), std::io::Error> {
        let to = to.join("common");
        if !to.is_dir() {
            fs::create_dir(&to)?;
        }

        fs::write(to.join(self.file_name), self.content)
    }

    /// Copy all stylesheets to the specified directory.
    ///
    /// # Errors
    ///
    /// Will return an error when there is a problem with writing the files or creating the directory.
    pub fn copy_all(to: &std::path::Path) -> Result<(), std::io::Error> {
        Stylesheet::TAILWIND.copy(to)
    }
}
