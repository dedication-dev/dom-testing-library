use crate::dom::css_selector::CSSSelector;
use crate::dom::element::Element;

#[cfg_attr(test, mockall::automock)]
pub trait Matcher {
    fn css_selectors(&self) -> Vec<CSSSelector>;

    fn matches(&self, element: &dyn Element) -> bool;
}

#[cfg(test)]
pub mod test_helper {
    use super::*;
    use crate::dom::attribute::Attribute;

    pub fn matching_matcher() -> MockMatcher {
        let mut matcher = MockMatcher::new();
        matcher.expect_css_selectors().return_const(vec![]);
        matcher.expect_matches().return_const(true);
        matcher
    }

    pub struct AttributeMatcher {
        attribute: Attribute,
    }

    impl AttributeMatcher {
        pub fn new(attribute: Attribute) -> Self {
            Self { attribute }
        }
    }

    impl Matcher for AttributeMatcher {
        fn css_selectors(&self) -> Vec<CSSSelector> {
            vec![]
        }

        fn matches(&self, element: &dyn Element) -> bool {
            match element.attribute(self.attribute.identifier()) {
                None => false,
                Some(attribute) => self.attribute == attribute,
            }
        }
    }
}
