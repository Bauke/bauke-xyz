use std::{fs, path::Path};

use {
  askama::Template,
  color_eyre::{eyre::eyre, Result},
  serde::Deserialize,
};

mod filters;

#[derive(Debug, Template)]
#[template(path = "video.html")]
pub struct VideoTemplate {
  /// The title of the page.
  pub page_title: String,

  /// Markdown rendered by [`comrak`].
  pub rendered_markdown: String,

  /// Data for a speedrun video.
  pub speedrun: Option<SpeedrunData>,

  /// The YouTube video ID.
  pub video_id: String,
}

#[derive(Debug, Deserialize)]
pub struct VideoData {
  /// The YouTube video ID.
  pub id: String,

  /// The title of the page.
  pub page_title: String,

  /// Data for a speedrun video.
  pub speedrun: Option<SpeedrunData>,

  /// Tags for the video.
  #[serde(default)]
  pub tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct SpeedrunData {
  /// Video chapters as with timestamps and chapter titles.
  pub chapters: Option<Vec<(String, String)>>,

  /// A link to the entry for this specific speedrun.
  pub entry: String,

  /// A link to the leaderboard for this speedrun's category.
  pub leaderboard: String,

  /// Deep Rock Galactic mods used in the run.
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

        let file_contents = fs::read_to_string(&file_path)?;
        let (video_data, markdown) =
          match toml_frontmatter::parse::<VideoData>(&file_contents) {
            Ok(parsed) => parsed,
            Err(error) => {
              println!("{:?} {}", file_path.file_name().unwrap(), error);
              continue;
            }
          };
        data.push((video_data, markdown.to_string()));
      }
    }

    data
  };

  let video_dir = public_dir.join("v");
  let expected_video_count = video_datas.len();

  for (video_data, markdown) in video_datas {
    let video_dir = video_dir.join(&video_data.id.to_lowercase());
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

    fs::write(
      video_dir.join("index.html"),
      crate::minify::html(template.render()?)?,
    )?;
  }

  let actual_video_count = fs::read_dir(video_dir)?.count();
  if expected_video_count != actual_video_count {
    return Err(eyre!(
      "Expected {} videos, found {}",
      expected_video_count,
      actual_video_count
    ));
  }

  Ok(())
}
