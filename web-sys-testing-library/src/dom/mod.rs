//! [web_sys] to [dom_testing_library] adapters.

mod document;
mod element;
mod node_list;

pub use document::Document;
pub use element::{Element, VecElementExt};
use node_list::NodeList;
