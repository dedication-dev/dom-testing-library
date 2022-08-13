use crate::dom::css_selector::CSSSelector;
use crate::dom::element::Element;

#[cfg_attr(test, mockall::automock)]
pub trait Queryable<TElement: Element> {
    fn query_all(&self, selectors: Vec<CSSSelector>) -> Vec<TElement>;
}
