use crate::dom::{CSSSelector, Element};

pub mod role;

pub trait Matcher {
    fn css_selectors(&self) -> Vec<CSSSelector>;

    fn matches(&self, element: &dyn Element) -> bool;
}

#[cfg(test)]
pub mod test_helper {
    use super::*;
    use crate::dom::Attribute;

    #[derive(Clone, Debug)]
    pub struct MatchingMatcher;

    impl Matcher for MatchingMatcher {
        fn css_selectors(&self) -> Vec<CSSSelector> {
            vec![]
        }

        fn matches(&self, _element: &dyn Element) -> bool {
            true
        }
    }

    #[derive(Clone, Debug, PartialEq)]
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
