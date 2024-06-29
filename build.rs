use std::{collections::BTreeMap, env, io::Write, path::Path, process::Command};

use anyhow::{Context, Result};
use convert_case::{Case, Casing};
use serde::Deserialize;

#[derive(Deserialize)]
struct Phosphor {
    #[serde(rename = "iconSets")]
    pub icon_sets: Vec<IconSet>,
}

#[derive(Deserialize)]
struct IconSet {
    pub icons: Vec<Icon>,
}

#[derive(Deserialize)]
struct Icon {
    pub tags: Vec<String>,
}

fn phosphor_icons<W: Write>(output: &mut W) -> Result<()> {
    let mut sorted_icons = BTreeMap::new();

    let weights: BTreeMap<String, String> = vec![
        ("THIN".to_string(), "ph-thin".to_string()),
        ("LIGHT".to_string(), "ph-light".to_string()),
        ("REGULAR".to_string(), "ph".to_string()),
        ("BOLD".to_string(), "ph-bold".to_string()),
        ("FILL".to_string(), "ph-fill".to_string()),
        ("DUOTONE".to_string(), "ph-duotone".to_string()),
    ]
    .into_iter()
    .collect::<BTreeMap<_, _>>();

    let config =
        serde_json::from_str::<Phosphor>(include_str!("phosphor-icons/src/Phosphor.json"))?;

    let icon_set = config.icon_sets.first().expect("No icon set found");

    for icon in &icon_set.icons {
        for lower_kebab_case in &icon.tags {
            let mut upper_snake_case = lower_kebab_case
                .from_case(Case::Kebab)
                .to_case(Case::UpperSnake);

            if upper_snake_case
                .chars()
                .all(|c| c == '_' || c.is_ascii_alphanumeric())
            {
                let first = upper_snake_case.chars().next().unwrap();
                if !first.is_ascii_alphabetic() && first != '_' {
                    upper_snake_case.insert(0, '_');
                }

                for (weight, class) in &weights {
                    let key = if weight == "REGULAR" {
                        upper_snake_case.to_string()
                    } else {
                        format!("{}_{}", upper_snake_case, weight)
                    };

                    sorted_icons.insert(
                        key.clone(),
                        format!(
                            "    pub const {key}: Icon = Icon(\"<i class=\\\"{class} ph-{lower_kebab_case}\\\"></i>\");\n"
                        ),
                    );
                }
            }
        }
    }

    output.write_all("#[allow(missing_docs)]\n".as_bytes())?;
    output.write_all("impl Icon {\n".as_bytes())?;
    sorted_icons
        .values()
        .try_for_each(|i| output.write_all(i.as_bytes()))?;
    output.write_all("}\n".as_bytes())?;

    Ok(())
}

fn tailwind_css(path: &Path) -> Result<()> {
    Command::new("pnpm")
        .args(&["install"])
        .status()
        .context("Failed to run `pnpm install`")?;

    let status = Command::new("pnpm")
        .args(&[
            "tailwindcss",
            "-i",
            "./assets/stylesheets/application.css",
            "-o",
            path.to_str().unwrap(),
        ])
        .status()
        .context("Failed to run tailwindcss")?;

    if !status.success() {
        return Err(anyhow::anyhow!("Failed to generate css"));
    }

    Ok(())
}

fn main() -> Result<()> {
    let out_dir = env::var_os("OUT_DIR").context("OUT_DIR not found")?;
    let icons_dest_path = Path::new(&out_dir).join("phosphor_icons_generated.rs");
    let css_dest_path = Path::new(&out_dir).join("tailwind_generated.css");

    phosphor_icons(&mut std::fs::File::create(icons_dest_path)?)
        .context("Failed to generate icons")?;

    println!("cargo::rerun-if-changed=src");
    tailwind_css(&css_dest_path).context("Failed to generate css")?;

    Ok(())
}
