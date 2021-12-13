use std::{fs, path::Path, process::Command};

use color_eyre::Result;

pub struct Copy;

impl Copy {
  pub fn write(
    build_dir: &Path,
    public_dir: &Path,
    source_dir: &Path,
  ) -> Result<()> {
    let files_to_copy = vec![(
      source_dir.join("netlify/_redirects"),
      public_dir.join("_redirects"),
    )];

    for (source, destination) in files_to_copy {
      fs::copy(source, destination)?;
    }

    let dirs_to_copy = vec![
      (source_dir.join("js"), &public_dir),
      (build_dir.join("userstyles"), &public_dir),
    ];

    for (source, destination) in dirs_to_copy {
      Command::new("cp")
        .arg("-r")
        .arg(source)
        .arg(destination)
        .output()?;
    }

    Ok(())
  }
}
