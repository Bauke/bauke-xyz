#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::missing_docs_in_private_items)]

//! # [bauke.xyz](https://bauke.xyz)

use yew::prelude::*;

/// Components collection.
pub(crate) mod components;

use components::{PageFooter, PageHeader, PageMain};

/// The main component.
pub(crate) struct Model;

impl Component for Model {
  type Message = ();
  type Properties = ();

  fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
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

/// Our main function.
pub(crate) fn main() {
  wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
  log::debug!("Initializing Yew");
  yew::start_app::<Model>();
}
