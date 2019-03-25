//! The `dodrio` virtual DOM.
//!
//! ## Example
//!
//! ```no_run
//! use dodrio::{bumpalo, Attribute, Node, Render, RenderContext};
//! use wasm_bindgen::UnwrapThrowExt;
//!
//! /// A component that greets someone.
//! pub struct Hello<'who> {
//!     who: &'who str,
//! }
//!
//! impl<'who> Hello<'who> {
//!     /// Construct a new `Hello` component that greets the given `who`.
//!     pub fn new(who: &str) -> Hello {
//!         Hello { who }
//!     }
//! }
//!
//! impl<'who> Render for Hello<'who> {
//!     fn render<'bump>(&self, cx: &mut RenderContext<'bump>) -> Node<'bump> {
//!         use dodrio::builder::*;
//!
//!         let id = bumpalo::format!(in cx.bump, "hello-{}", self.who);
//!         let who = bumpalo::collections::String::from_str_in(self.who, cx.bump).into_bump_str();
//!
//!         div(&cx)
//!            .attr("id", id.into_bump_str())
//!            .on("click", |root, _vdom, _event| {
//!                 let hello = root.unwrap_mut::<Hello>();
//!                 web_sys::window()
//!                     .expect_throw("should have a `Window` on the Web")
//!                     .alert_with_message(hello.who);
//!             })
//!             .children([
//!                 text("Hello, "),
//!                 strong(&cx)
//!                     .children([
//!                         text(who),
//!                         text("!"),
//!                     ])
//!                     .finish(),
//!             ])
//!             .finish()
//!     }
//! }
//! ```
#![deny(missing_docs, missing_debug_implementations)]

// Re-export the `bumpalo` crate.
pub use bumpalo;

cfg_if::cfg_if! {
    if #[cfg(feature = "log")] {
        #[macro_use]
        extern crate log;
    } else {
        #[macro_use]
        mod logging;
    }
}

// Only `pub` so that the wasm-bindgen bindings work.
#[doc(hidden)]
pub mod change_list;

mod events;
mod node;
mod render;
mod render_context;
mod vdom;

pub mod builder;

// Re-export items at the top level.
pub use self::node::{Attribute, ElementNode, Listener, ListenerCallback, Node, TextNode};
pub use self::render::{Render, RootRender};
pub use self::render_context::RenderContext;
pub use self::vdom::{Vdom, VdomWeak};

cfg_if::cfg_if! {
    if #[cfg(feature = "xxx-unstable-internal-use-only")] {
        /// An element node in the physical DOM.
        pub type Element = ();

        pub(crate) type EventsTrampoline = ();
    } else {
        /// An element node in the physical DOM.
        pub type Element = web_sys::Element;

        pub(crate) type EventsTrampoline = wasm_bindgen::closure::Closure<Fn(web_sys::Event, u32, u32)>;
    }
}
