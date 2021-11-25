use yew::prelude::*;

/// The main page `<footer>` element.
pub(crate) struct PageFooter;

impl Component for PageFooter {
  type Message = ();
  type Properties = ();

  fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
    log::trace!("Creating PageFooter");

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
      <footer class="page-footer"></footer>
    }
  }
}
