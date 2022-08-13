use crate::dom::element::Element;
use crate::dom::queryable::Queryable;
use crate::query::matcher::Matcher;

pub trait QueryAllBy<TElement: Element> {
    fn query_all_by<TMatcher: Matcher>(&self, matcher: &TMatcher) -> Vec<TElement>;
}

impl<TElement: Element, TQueryable: Queryable<TElement>> QueryAllBy<TElement> for TQueryable {
    fn query_all_by<TMatcher: Matcher>(&self, matcher: &TMatcher) -> Vec<TElement> {
        self.query_all(matcher.css_selectors())
            .into_iter()
            .filter(|it| matcher.matches(it))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod query_all_by {
        use super::*;
        use crate::dom::css_selector::CSSSelector;
        use crate::dom::element::{Attribute, AttributeIdentifier, Element};
        use crate::dom::queryable::MockQueryable;
        use crate::query::matcher::MockMatcher;
        use std::collections::HashMap;

        #[test]
        fn with_query_all_without_elements_returns_no_elements() {
            let elements = vec![];
            let queryable = non_filtering_queryable(&elements);

            let matching_elements = queryable.query_all_by(&matching_matcher());

            assert_eq!(matching_elements, elements);
        }

        #[test]
        fn returns_matching_elements() {
            let elements = vec![FakeElement::default()];
            let queryable = non_filtering_queryable(&elements);

            let matching_elements = queryable.query_all_by(&matching_matcher());

            assert_eq!(matching_elements, elements);
        }

        #[test]
        fn does_not_return_non_matching_elements() {
            let other_attribute = Attribute::Valueless {
                identifier: "attr1".into(),
            };
            let element1 = FakeElement::new(AttributeMap::from(vec![other_attribute]));
            let matching_attribute = Attribute::Valueless {
                identifier: "attr2".into(),
            };
            let element2 = FakeElement::new(AttributeMap::from(vec![matching_attribute.clone()]));
            let queryable = non_filtering_queryable(&[element1, element2.clone()]);

            let matcher = AttributeMatcher::new(matching_attribute);
            let matching_elements = queryable.query_all_by(&matcher);

            assert_eq!(matching_elements, vec![element2]);
        }

        fn non_filtering_queryable(elements: &[FakeElement]) -> MockQueryable<FakeElement> {
            let mut queryable = MockQueryable::new();
            queryable.expect_query_all().return_const(elements.to_vec());
            queryable
        }

        #[derive(Clone, Debug, Default, PartialEq)]
        struct AttributeMap(HashMap<AttributeIdentifier, Attribute>);

        impl From<Vec<Attribute>> for AttributeMap {
            fn from(attributes: Vec<Attribute>) -> Self {
                Self(HashMap::from_iter(attributes.into_iter().map(
                    |attribute| (attribute.identifier().clone(), attribute),
                )))
            }
        }

        #[derive(Clone, Debug, Default, PartialEq)]
        struct FakeElement {
            attributes: AttributeMap,
        }

        impl FakeElement {
            fn new(attributes: AttributeMap) -> Self {
                Self { attributes }
            }
        }

        impl Element for FakeElement {
            fn attribute(&self, identifier: &AttributeIdentifier) -> Option<Attribute> {
                self.attributes.0.get(identifier).cloned()
            }
        }

        fn matching_matcher() -> MockMatcher {
            let mut matcher = MockMatcher::new();
            matcher.expect_css_selectors().return_const(vec![]);
            matcher.expect_matches().return_const(true);
            matcher
        }

        struct AttributeMatcher {
            attribute: Attribute,
        }

        impl AttributeMatcher {
            fn new(attribute: Attribute) -> Self {
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
}