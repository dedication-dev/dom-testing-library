use crate::dom::css_selector::CSSSelector;
use crate::dom::element::Element;

#[cfg_attr(test, mockall::automock)]
pub trait Matcher {
    fn css_selectors(&self) -> Vec<CSSSelector>;

    fn matches(&self, element: &dyn Element) -> bool;
}
