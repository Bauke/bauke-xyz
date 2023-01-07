/*!
Filters for Askama templates.
*/

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
