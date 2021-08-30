#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::missing_docs_in_private_items)]

//! # [bauke.xyz](https://bauke.xyz)

use yew::prelude::*;
use yew_router::router::Router;

/// Components collection.
pub(crate) mod components;
/// Routes collection.
pub(crate) mod routes;

/// All routes.
#[derive(Clone, yew_router::Switch)]
pub(crate) enum Route {
  #[to = "/userstyles"]
  Userstyles,
  #[to = "/{}"]
  NotFound(String),
  #[to = "/"]
  Home,
}

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
      <Router<Route, ()>
        render = Router::render(|route: Route| {
          match route {
            Route::NotFound(_) => html! {
              <main class="error-404">
                <p>{"🤷"}</p>
              </main>
            },
            Route::Home => html! {
              <routes::HomeRoute />
            },
            Route::Userstyles => html! {
              <routes::UserstylesRoute />
            }
          }
        })
      />
    }
  }
}

/// Our main function.
pub(crate) fn main() {
  wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
  log::debug!("Initializing Yew");
  yew::start_app::<Model>();
}
