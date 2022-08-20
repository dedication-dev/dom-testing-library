use crate::dom::css_selector::CSSSelector;
use crate::dom::element::Element;
use crate::query::matcher::Matcher;

pub struct RoleButton(());

impl Matcher for RoleButton {
    fn css_selectors(&self) -> Vec<CSSSelector> {
        vec![CSSSelector::exact_attribute_selector(
            "role".into(),
            "button".into(),
        )]
    }

    fn matches(&self, _element: &dyn Element) -> bool {
        true
    }
}

pub fn button() -> RoleButton {
    RoleButton(())
}
