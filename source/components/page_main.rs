use yew::prelude::*;

/// The main page `<main>` element.
pub(crate) struct PageMain;

impl Component for PageMain {
  type Message = ();
  type Properties = ();

  fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
    log::trace!("Creating PageMain");

    Self
  }

  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    unimplemented!()
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    html! {
      <main class="page-main">
        <div class="contact-links">
          <a target="_blank" href="mailto:me@bauke.xyz">
            {"me@bauke.xyz"}
          </a>

          <a target="_blank" href="https://matrix.to/#/@baukexyz:matrix.org">
            {"@baukexyz:matrix.org"}
          </a>

          <a target="_blank" href="https://mastodon.social/@bauke">
            {"@bauke@mastodon.social"}
          </a>
        </div>
      </main>
    }
  }
}
