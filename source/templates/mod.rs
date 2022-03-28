use std::{
  fs::{create_dir_all, write},
  path::Path,
};

use askama::Template;
use color_eyre::Result;

#[derive(Debug, Template)]
#[template(path = "index.html")]
pub struct Index {
  pub page_title: String,
}

impl Index {
  pub fn write(public_dir: &Path) -> Result<()> {
    let destination = public_dir.join("index.html");
    create_dir_all(destination.parent().unwrap())?;

    let template = Self {
      page_title: "Bauke".to_string(),
    };

    write(destination, template.render()?)?;

    Ok(())
  }
}

#[derive(Debug, Template)]
#[template(path = "userstyles.html")]
pub struct Userstyles {
  pub page_title: String,
  pub styles: Vec<userstyles::Userstyle>,
}

impl Userstyles {
  pub fn write(public_dir: &Path) -> Result<()> {
    let destination = public_dir.join("userstyles/index.html");
    create_dir_all(destination.parent().unwrap())?;

    let styles = userstyles::ALL_USERSTYLES
      .iter()
      .map(|target| userstyles::Userstyle::load(target))
      .flatten()
      .collect();

    let template = Self {
      page_title: "Bauke".to_string(),
      styles,
    };

    write(destination, template.render()?)?;

    Ok(())
  }
}
