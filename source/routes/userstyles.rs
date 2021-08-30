use yew::prelude::*;

use crate::components::PageHeader;

/// The route for `/userstyles`.
pub(crate) struct UserstylesRoute;

impl Component for UserstylesRoute {
  type Message = ();
  type Properties = ();

  fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
    log::trace!("Creating UserstylesRoute");

    Self
  }

  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    unimplemented!()
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    let styles = userstyles::ALL_USERSTYLES
      .iter()
      .map(|target| userstyles::Userstyle::load(target))
      .flatten()
      .map(|style| {
        let style_name = style.metadata.name.to_lowercase().replace(" ", "-");

        html! {
          <div class="style">
            <div class="header">
              <img src=format!("/userstyles/{}.png", style_name) />
              <h2>{style.metadata.name}</h2>
              <a target="_blank" href={style.metadata.update_url}>{"Install"}</a>
            </div>

            <p>{style.metadata.description}</p>
          </div>
        }
      })
      .collect::<Vec<_>>();

    html! {
      <>
        <PageHeader />
        <main class="page-main userstyles">
          {styles}
        </main>
      </>
    }
  }
}
