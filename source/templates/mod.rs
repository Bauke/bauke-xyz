use askama::Template;

#[derive(Debug, Template)]
#[template(path = "index.html")]
pub struct Index {
  pub page_title: String,
}

#[derive(Debug, Template)]
#[template(path = "userstyles.html")]
pub struct Userstyles {
  pub page_title: String,
  pub styles: Vec<userstyles::Userstyle>,
}
