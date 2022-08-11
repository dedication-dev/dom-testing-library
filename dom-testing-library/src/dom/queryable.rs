use crate::dom::css_selector::CSSSelector;
use crate::dom::node::Node;

#[cfg_attr(test, mockall::automock)]
pub trait Queryable<TNode: Node> {
    fn query_all(&self, selectors: Vec<CSSSelector>) -> Vec<TNode>;
}
