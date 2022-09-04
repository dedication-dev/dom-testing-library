#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

pub mod dom;
mod query;
mod render;

pub use query::{GetAllByError, GetAllByResult};
pub use render::render_html;

/// Reexports [dom_testing_library::query::matcher]
pub mod matcher {
    pub use dom_testing_library::query::matcher::*;
}
