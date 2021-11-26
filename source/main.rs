use std::{fs, path::PathBuf, process::Command};

use askama::Template;

mod templates;

fn main() -> color_eyre::Result<()> {
  color_eyre::install()?;

  let build_dir = PathBuf::from("target");

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

  let public_dir = PathBuf::from("public");

  let source_dir = PathBuf::from("source");

  let styles = userstyles::ALL_USERSTYLES
    .iter()
    .map(|target| userstyles::Userstyle::load(target))
    .flatten()
    .collect::<Vec<_>>();

  let templates_to_render: Vec<(Box<dyn Template>, PathBuf)> = vec![
    (
      Box::new(templates::Index {
        page_title: "bauke.xyz".to_string(),
      }),
      public_dir.join("index.html"),
    ),
    (
      Box::new(templates::Userstyles {
        page_title: "bauke.xyz".to_string(),
        styles,
      }),
      public_dir.join("userstyles/index.html"),
    ),
  ];

  for (template, path) in templates_to_render {
    fs::create_dir_all(&path.parent().unwrap())?;
    let rendered = template.render()?;
    fs::write(path, rendered)?;
  }

  let css_dir = public_dir.join("css");
  fs::create_dir_all(&css_dir)?;

  let scss_dir = source_dir.join("scss");

  let scss_to_render = vec![
    (scss_dir.join("index.scss"), css_dir.join("index.css")),
    (
      scss_dir.join("modern-normalize.scss"),
      css_dir.join("modern-normalize.css"),
    ),
  ];

  for (source, destination) in scss_to_render {
    let rendered = rsass::compile_scss_path(
      &source,
      rsass::output::Format {
        style: rsass::output::Style::Expanded,
        precision: 5,
      },
    )?;

    fs::write(destination, rendered)?;
  }

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
