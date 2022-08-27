#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

mod dom;
mod query;
mod render;

pub use query::QueryAllBy;
pub use render::render_html;

/// Reexports [dom_testing_library::query::matcher]
pub mod matcher {
    pub use dom_testing_library::query::matcher::*;
}
