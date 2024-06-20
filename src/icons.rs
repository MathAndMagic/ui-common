//! # Icons
//!
//! Based on https://github.com/isosphere/yew-bootstrap/blob/main/packages/yew-bootstrap/src/icons/mod.rs
//!
//! All icons are available as a constant, thanks to the build-executed script:
//! ```
//! # use yew::html;
//! # use ui_common::Icon;
//! let icon = Icon::HEART;
//! # let result =
//! html!{
//!     <h1>{"I"} {icon} {Icon::GEAR}</h1>
//! }
//! # ;
//! ```
//!
//! # Required CSS
//!
//! These icons require the inclusion of a CSS file from Phosphor (`style.css`). This file can be added by:
//!
//! 1. Using [`IconFiles::copy()`](crate::icons::IconFiles::copy) - in build-executed code (recommended, see below)
//! 2. Accessing the data via [`IconFiles::FILES`](crate::icons::IconFiles::FILES) and delivering it yourself
//!
//! # 1. Recommended - Automatically Copying Files
//!
//! This is copies the files to the `dist` directory, which is recommended.
//!
//! It is shown in `/examples/icons`.
//!
//! A copy of `phosphor-icons` is included and should change only rarely. `trunk` does not add a hash to generated files,
//! and thus a change in those files won't be detected by `trunk`.
//!
//! 1. `Cargo.toml`
//!
//!    Add the build-executed binary.
//!
//!    ```toml
//!    [[bin]]
//!    name = "copy-icons"
//!    ```
//!
//! 2. `src/bin/copy-icons.rs`
//!
//!    Create the program to copy the files.
//!
//!    ```no_run
//!    use std::path::PathBuf;
//!    use ui_common::IconFiles;
//!
//!    fn main() -> Result<(), std::io::Error> {
//!        let path = PathBuf::from(
//!            std::env::var("TRUNK_STAGING_DIR").expect("Environment variable TRUNK_STAGING_DIR"),
//!        );
//!        if !path.is_dir() {
//!            std::fs::create_dir(&path)?;
//!        }
//!
//!        IconFiles::FILES.copy(&path)
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
//!    <link rel="stylesheet" href="phosphor-icons/regular/style.css" />
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
//!    command_arguments = ["run", "--bin", "copy-icons"]
//!    ```
//!
use std::{
    fs,
    hash::{Hash, Hasher},
};

use yew::{
    html::{ChildrenRenderer, IntoPropValue},
    virtual_dom::{VNode, VRaw},
    AttrValue, Html, ToHtml,
};

#[derive(Clone, Copy, Ord, PartialOrd, Eq, Debug)]
#[repr(transparent)]
pub struct Icon(pub(crate) &'static str);

impl Icon {
    /// Returns the `Html` of this icon.
    #[inline]
    pub const fn html(self) -> Html {
        VNode::VRaw(VRaw {
            html: AttrValue::Static(self.0),
        })
    }

    /// Returns the raw html as a str of this icon.
    #[inline]
    #[must_use]
    pub const fn raw_html(self) -> &'static str {
        self.0
    }
}

impl PartialEq for Icon {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        // Invariant: All possible strings are different and thus `(ptr,len)` must me different as well.
        // Invariant: No two strings at different pointers are equal,
        // Invariant: this is guaranteed due to the fact that it's not possible to create new.
        // Performance hack: Only check those.
        self.0.as_ptr() as usize == other.0.as_ptr() as usize && self.0.len() == other.0.len()
    }
}

impl Hash for Icon {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Invariant: All possible strings are different and thus `(ptr,len)` must be different as well.
        // Invariant: No two strings at different pointers are equal,
        // Invariant: this is guaranteed due to the fact that it's not possible to create new.
        // Performance hack: Only hash the ptr to the middle of the string.
        (self.0.as_ptr() as usize + (self.0.len() >> 1)).hash(state);
    }
}

impl From<Icon> for Html {
    #[inline]
    fn from(value: Icon) -> Self {
        value.html()
    }
}

impl From<&Icon> for Html {
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn from(value: &Icon) -> Self {
        Html::from(*value)
    }
}

impl IntoPropValue<ChildrenRenderer<VNode>> for Icon {
    #[inline]
    fn into_prop_value(self) -> ChildrenRenderer<VNode> {
        self.html().into_prop_value()
    }
}

impl IntoPropValue<ChildrenRenderer<VNode>> for &Icon {
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn into_prop_value(self) -> ChildrenRenderer<VNode> {
        (*self).into_prop_value()
    }
}

impl ToHtml for Icon {
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn to_html(&self) -> Html {
        self.html()
    }
}

/// Holds all phosphor-icons data.
///
/// Intended use:
/// ```
/// # use ui_common::IconFiles;
/// let IconFiles {variant, css, svg, font_ttf, font_woff, font_woff2, license} = IconFiles::FILES;
/// ```
///
/// (That way it will be an error if a file is added/removed.)
pub struct IconFiles {
    /// Name of the variant.
    pub variant: &'static str,
    /// Contents of the file `style.css`.
    pub css: &'static str,
    /// Contents of the file `Phosphor-*.svg`.
    pub svg: &'static str,
    /// Contents of the file `Phosphor-*.ttf`.
    pub font_ttf: &'static [u8],
    /// Contents of the file `Phosphor-*.woff`.
    pub font_woff: &'static [u8],
    /// Contents of the file `Phosphor-*.woff2`.
    pub font_woff2: &'static [u8],
    /// Contents of the file `LICENSE`.
    pub license: &'static str,
}

impl IconFiles {
    /// Holds all phosphor-icons data for the `regular` variant.
    ///
    /// Intended use:
    /// ```
    /// # use ui_common::IconFiles;
    /// let IconFiles {variant, css, svg, font_ttf, font_woff, font_woff2, license} = IconFiles::FILES;
    /// ```
    /// (That way it will be an error if a file is added/removed.)
    pub const FILES: Self = Self {
        variant: "Regular",
        css: include_str!("../phosphor-icons/src/regular/style.css"),
        svg: include_str!("../phosphor-icons/src/regular/Phosphor.svg"),
        font_ttf: include_bytes!("../phosphor-icons/src/regular/Phosphor.ttf"),
        font_woff: include_bytes!("../phosphor-icons/src/regular/Phosphor.woff"),
        font_woff2: include_bytes!("../phosphor-icons/src/regular/Phosphor.woff2"),
        license: include_str!("../phosphor-icons/LICENSE"),
    };

    /// Holds all phosphor-icons data for the `bold` variant.
    ///
    /// Intended use: see [`IconFiles::FILES`].
    pub const FILES_BOLD: Self = Self {
        variant: "Bold",
        css: include_str!("../phosphor-icons/src/bold/style.css"),
        svg: include_str!("../phosphor-icons/src/bold/Phosphor-Bold.svg"),
        font_ttf: include_bytes!("../phosphor-icons/src/bold/Phosphor-Bold.ttf"),
        font_woff: include_bytes!("../phosphor-icons/src/bold/Phosphor-Bold.woff"),
        font_woff2: include_bytes!("../phosphor-icons/src/bold/Phosphor-Bold.woff2"),
        license: include_str!("../phosphor-icons/LICENSE"),
    };

    /// Holds all phosphor-icons data for the `duotone` variant.
    ///
    /// Intended use: see [`IconFiles::FILES`].
    pub const FILES_DUOTONE: Self = Self {
        variant: "Duotone",
        css: include_str!("../phosphor-icons/src/duotone/style.css"),
        svg: include_str!("../phosphor-icons/src/duotone/Phosphor-Duotone.svg"),
        font_ttf: include_bytes!("../phosphor-icons/src/duotone/Phosphor-Duotone.ttf"),
        font_woff: include_bytes!("../phosphor-icons/src/duotone/Phosphor-Duotone.woff"),
        font_woff2: include_bytes!("../phosphor-icons/src/duotone/Phosphor-Duotone.woff2"),
        license: include_str!("../phosphor-icons/LICENSE"),
    };

    /// Holds all phosphor-icons data for the `fill` variant.
    ///
    /// Intended use: see [`IconFiles::FILES`].
    pub const FILES_FILL: Self = Self {
        variant: "Fill",
        css: include_str!("../phosphor-icons/src/fill/style.css"),
        svg: include_str!("../phosphor-icons/src/fill/Phosphor-Fill.svg"),
        font_ttf: include_bytes!("../phosphor-icons/src/fill/Phosphor-Fill.ttf"),
        font_woff: include_bytes!("../phosphor-icons/src/fill/Phosphor-Fill.woff"),
        font_woff2: include_bytes!("../phosphor-icons/src/fill/Phosphor-Fill.woff2"),
        license: include_str!("../phosphor-icons/LICENSE"),
    };

    /// Holds all phosphor-icons data for the `light` variant.
    ///
    /// Intended use: see [`IconFiles::FILES`].
    pub const FILES_LIGHT: Self = Self {
        variant: "Light",
        css: include_str!("../phosphor-icons/src/light/style.css"),
        svg: include_str!("../phosphor-icons/src/light/Phosphor-Light.svg"),
        font_ttf: include_bytes!("../phosphor-icons/src/light/Phosphor-Light.ttf"),
        font_woff: include_bytes!("../phosphor-icons/src/light/Phosphor-Light.woff"),
        font_woff2: include_bytes!("../phosphor-icons/src/light/Phosphor-Light.woff2"),
        license: include_str!("../phosphor-icons/LICENSE"),
    };

    /// Holds all phosphor-icons data for the `thin` variant.
    ///
    /// Intended use: see [`IconFiles::FILES`].
    pub const FILES_THIN: Self = Self {
        variant: "Thin",
        css: include_str!("../phosphor-icons/src/thin/style.css"),
        svg: include_str!("../phosphor-icons/src/thin/Phosphor-Thin.svg"),
        font_ttf: include_bytes!("../phosphor-icons/src/thin/Phosphor-Thin.ttf"),
        font_woff: include_bytes!("../phosphor-icons/src/thin/Phosphor-Thin.woff"),
        font_woff2: include_bytes!("../phosphor-icons/src/thin/Phosphor-Thin.woff2"),
        license: include_str!("../phosphor-icons/LICENSE"),
    };

    /// Copy all phosphor icons files to the specified directory.
    ///
    /// # Errors
    ///
    /// Will return an error when there is a problem with creating the directories or writing the files.
    pub fn copy(&self, to: &std::path::Path) -> Result<(), std::io::Error> {
        let IconFiles {
            variant,
            css,
            svg,
            font_ttf,
            font_woff,
            font_woff2,
            license,
        } = self;

        let icons = to.join("phosphor-icons");
        if !icons.is_dir() {
            fs::create_dir(&icons)?;
        }

        let to = icons.join(variant.to_lowercase());
        if !to.is_dir() {
            fs::create_dir(&to)?;
        }

        fs::write(to.join("style.css"), css)?;
        fs::write(to.join(format!("Phosphor-{}.svg", variant)), svg)?;
        fs::write(to.join(format!("Phosphor-{}.ttf", variant)), font_ttf)?;
        fs::write(to.join(format!("Phosphor-{}.woff", variant)), font_woff)?;
        fs::write(to.join(format!("Phosphor-{}.woff2", variant)), font_woff2)?;
        fs::write(to.join("LICENSE"), license)?;

        Ok(())
    }
}

include!(concat!(env!("OUT_DIR"), "/phosphor_icons_generated.rs"));
