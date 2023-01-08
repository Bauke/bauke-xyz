use std::{fs, path::Path};

use {askama::Template, color_eyre::Result, serde::Deserialize};

mod filters;

#[derive(Debug, Template)]
#[template(path = "video.html")]
pub struct VideoTemplate {
  pub page_title: String,
  pub rendered_markdown: String,
  pub speedrun: Option<SpeedrunData>,
  pub video_id: String,
}

#[derive(Debug, Deserialize)]
struct VideoData {
  pub id: String,
  pub page_title: String,
  pub speedrun: Option<SpeedrunData>,
}

#[derive(Debug, Deserialize)]
pub struct SpeedrunData {
  pub chapters: Option<Vec<(String, String)>>,
  pub entry: String,
  pub leaderboard: String,
  pub mods: Option<Vec<String>>,
}

pub fn write_all(public_dir: &Path) -> Result<()> {
  let video_datas = {
    let mut data = vec![];

    for dir in ["2022"] {
      for file in fs::read_dir(format!("source/video/{dir}"))? {
        let file_path = file?.path();
        if file_path.extension().unwrap() != "md" {
          continue;
        }

        let file_contents = fs::read_to_string(file_path)?;
        let (video_data, markdown) =
          match toml_frontmatter::parse::<VideoData>(&file_contents) {
            Ok(parsed) => parsed,
            Err(error) => {
              println!("{error}");
              continue;
            }
          };
        data.push((video_data, markdown.to_string()));
      }
    }

    data
  };

  for (video_data, markdown) in video_datas {
    let video_dir = public_dir.join("v").join(&video_data.id);
    fs::create_dir_all(&video_dir)?;

    let template = VideoTemplate {
      page_title: video_data.page_title,
      rendered_markdown: comrak::markdown_to_html(
        &markdown,
        &Default::default(),
      ),
      speedrun: video_data.speedrun,
      video_id: video_data.id,
    };
    fs::write(video_dir.join("index.html"), template.render()?)?;
  }

  Ok(())
}
