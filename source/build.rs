/// Build script for the website.

fn main() {
  println!("cargo:rerun-if-changed=source/**");
  let build_dir = std::path::PathBuf::from("target");

  for target in userstyles::ALL_USERSTYLES {
    let style = userstyles::Userstyle::load(target).unwrap();
    let style_name = style.metadata.name.to_lowercase().replace(" ", "-");

    let style_dir = build_dir.join("userstyles");
    std::fs::create_dir_all(&style_dir).unwrap();

    let style_file = style_dir.join(format!("{}.user.css", style_name));
    let formatted = style.format();
    std::fs::write(style_file, formatted).unwrap();

    if let Some(image) = style.image {
      let image_file = style_dir.join(format!("{}.png", style_name));
      std::fs::write(image_file, image).unwrap();
    }
  }
}
