use crate::dom::{CSSSelector, Element};
use crate::query::matcher::Matcher;

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RoleButton;

impl Matcher for RoleButton {
    fn css_selectors(&self) -> Vec<CSSSelector> {
        vec![CSSSelector::exact_attribute("role".into(), "button".into())]
    }

    fn matches(&self, _element: &dyn Element) -> bool {
        true
    }
}

pub fn button() -> RoleButton {
    RoleButton
}
