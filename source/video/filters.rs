/*!
Filters for Askama templates.
*/

/**
Get the DRG mod link and title from a given ID.
*/
pub fn drg_mod(mod_id: &str) -> askama::Result<(String, &str)> {
  let mods = std::collections::HashMap::<_, _>::from_iter([
    ("drglib", "DRGLib"),
    ("simplemissiontimer", "SimpleMissionTimer"),
  ]);

  let mod_title = mods.get(mod_id).unwrap();
  Ok((format!("https://drg.mod.io/{mod_id}"), mod_title))
}

/**
Turn a timestamp with format `mm:ss` into its total seconds.

## Examples

- `00:30` -> 30 seconds
- `01:00` -> 60 seconds
- `01:30` -> 90 seconds
*/
pub fn timestamp_to_seconds(timestamp: &str) -> askama::Result<i32> {
  let mut split = timestamp.split(":");
  let minutes = split.next().map(str::parse::<i32>).unwrap().unwrap();
  let seconds = split.next().map(str::parse::<i32>).unwrap().unwrap();
  Ok(minutes * 60 + seconds)
}
