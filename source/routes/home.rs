use yew::prelude::*;

use crate::components::{PageFooter, PageHeader, PageMain};

/// The route for `/`.
pub(crate) struct HomeRoute;

impl Component for HomeRoute {
  type Message = ();
  type Properties = ();

  fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
    log::trace!("Creating HomeRoute");

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
      <>
        <PageHeader />
        <PageMain />
        <PageFooter />
      </>
    }
  }
}
