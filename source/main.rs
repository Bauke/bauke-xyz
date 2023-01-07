use std::{
  fs,
  path::{Path, PathBuf},
};

use color_eyre::{install, Result};

mod copy;
mod scss;
mod templates;
mod video;

fn main() -> Result<()> {
  install()?;

  let build_dir = PathBuf::from("target");
  let public_dir = PathBuf::from("public");
  let source_dir = PathBuf::from("source");

  build_userstyles(&build_dir)?;
  templates::Index::write(&public_dir)?;
  scss::Scss::write(&public_dir, &source_dir)?;
  copy::Copy::write(&build_dir, &public_dir, &source_dir)?;
  video::write_all(&public_dir)?;

  Ok(())
}

fn build_userstyles(build_dir: &Path) -> Result<()> {
  for target in userstyles::ALL_USERSTYLES {
    let style = userstyles::Userstyle::load(target)?;
    let style_name = style.metadata.name.to_lowercase().replace(" ", "-");

    let style_dir = build_dir.join("userstyles");
    fs::create_dir_all(&style_dir)?;

    let style_file = style_dir.join(format!("{}.user.css", style_name));
    let formatted = style.format();
    fs::write(style_file, formatted)?;

    if let Some(image) = style.image {
      let image_file = style_dir.join(format!("{}.png", style_name));
      fs::write(image_file, image)?;
    }
  }

  Ok(())
}
