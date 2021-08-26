use gloo_timers::callback::Interval;
use rand::Rng;
use yew::prelude::*;

/// The main page `<footer>` element.
pub(crate) struct PageFooter {
  /// The handle to the `setInterval`, this is stored so it doesn't get dropped
  /// and stop the interval from happening.
  _interval_handle: Interval,
  /// The number of which square is currently active.
  active_square: u32,
  /// The link to this component.
  link: ComponentLink<Self>,
  /// The maximum number of squares to render.
  max_squares: u32,
}

/// The possible messages for [`PageFooter`].
#[derive(Debug)]
pub(crate) enum PageFooterMessage {
  /// The message to decrease the max squares by 1.
  DecrementMaxSquares,
  /// The message to increase the max squares by 1.
  IncrementMaxSquares,
  /// The message from the interval to update the active square.
  TickActiveSquare,
}

impl Component for PageFooter {
  type Message = PageFooterMessage;
  type Properties = ();

  fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
    log::trace!("Creating PageFooter");

    let interval_handle = {
      let link = link.clone();
      Interval::new(1000, move || {
        link.send_message(Self::Message::TickActiveSquare)
      })
    };

    Self {
      _interval_handle: interval_handle,
      active_square: 0,
      link,
      max_squares: 5,
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    log::trace!("PageFooterMessage::{:?}", msg);

    match msg {
      Self::Message::DecrementMaxSquares => {
        if self.max_squares > 1 {
          self.max_squares -= 1;
          log::debug!("Max squares set to {}", self.max_squares);
          true
        } else {
          false
        }
      }

      Self::Message::IncrementMaxSquares => {
        if self.max_squares < 360 {
          self.max_squares += 1;
          log::debug!("Max squares set to {}", self.max_squares);
          true
        } else {
          false
        }
      }

      Self::Message::TickActiveSquare => {
        if self.max_squares == 1 {
          if self.active_square == 0 {
            self.active_square = 1;
          } else {
            self.active_square = 0;
          }
        } else {
          self.active_square += 1;
          if self.active_square >= self.max_squares {
            self.active_square = 0;
          }
        }

        true
      }
    }
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    let squares = (0..self.max_squares)
      .into_iter()
      .map(|square| {
        let mut classes = classes!("square");
        if square == self.active_square {
          classes.push("active");
        }

        let hue = {
          if self.max_squares == 1 {
            rand::thread_rng().gen_range(0..=360)
          } else {
            (360 / self.max_squares) * square
          }
        };
        let style = format!("background-color: hsl({}, 100%, 50%);", hue);

        return html! {
          <div style=style class=classes />
        };
      })
      .collect::<Html>();

    let squares = {
      let style = format!(
        "--square-transition: {0}s; --squares: {0};",
        self.max_squares
      );
      let decrement = self
        .link
        .callback(|_| PageFooterMessage::DecrementMaxSquares);
      let increment = self
        .link
        .callback(|_| PageFooterMessage::IncrementMaxSquares);

      html! {
        <div style=style class="squares">
          <button onclick=decrement>{"-"}</button>
          {squares}
          <button onclick=increment>{"+"}</button>
        </div>
      }
    };

    let technologies = html! {
      <p class="technologies">
        {"Written in "}
        <a target="_blank" href="https://www.rust-lang.org">{"Rust"}</a>
        {" with "}
        <a target="_blank" href="https://yew.rs">{"Yew"}</a>
        {" and compiled to "}
        <a target="_blank" href="https://webassembly.org">{"WebAssembly"}</a>
        {" with "}
        <a target="_blank" href="https://trunkrs.dev">{"Trunk"}</a>
        {"."}
      </p>
    };

    html! {
      <footer class="page-footer">
        {squares}
        {technologies}
      </footer>
    }
  }
}
