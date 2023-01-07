use std::{
  fs::{create_dir_all, write},
  path::Path,
};

use color_eyre::Result;
use rsass::{
  compile_scss_path,
  output::{Format, Style},
};

pub struct Scss;

impl Scss {
  pub fn write(public_dir: &Path, source_dir: &Path) -> Result<()> {
    let css_dir = public_dir.join("css");
    create_dir_all(&css_dir)?;

    let scss_dir = source_dir.join("scss");
    let scss_filenames = vec!["index", "modern-normalize", "video"];

    let format = Format {
      precision: 5,
      style: Style::Compressed,
    };
    for filename in scss_filenames {
      let scss_path = scss_dir.join(format!("{}.scss", filename));
      let css = compile_scss_path(&scss_path, format)?;
      let css_path = css_dir.join(format!("{}.css", filename));
      write(css_path, css)?;
    }

    Ok(())
  }
}
