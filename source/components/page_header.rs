use yew::prelude::*;

/// The main page `<header>` element.
pub(crate) struct PageHeader;

impl Component for PageHeader {
  type Message = ();
  type Properties = ();

  fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
    log::trace!("Creating PageHeader");

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
      <header class="page-header">
        <h1>{"bauke.xyz"}</h1>
      </header>
    }
  }
}
